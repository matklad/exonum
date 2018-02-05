use std::panic;

use byteorder::{LittleEndian, ByteOrder};

use crypto::Hash;
use storage::db::{Database, Patch};
use consensus::ValidatorId;
use blockchain::{self, Height, BlockchainSchema, SCHEMA_MAJOR_VERSION, Block};
use ::App;
use transaction::Tx;

pub(crate) struct ExonumCore {
    pub db: Box<Database>,
    pub app: Box<App>,
}

impl ExonumCore {
    /// Executes the given transactions from pool.
    /// Then it collects the resulting changes from the current storage state and returns them
    /// with the hash of resulting block.
    pub fn create_patch(
        &self,
        proposer_id: ValidatorId,
        height: Height,
        transactions: Vec<Tx>,
    ) -> (Hash, Patch) {

        // Create a fork
        let mut fork = self.db.fork();
        let tx_hash = blockchain::execute_transactions(
            &mut fork,
            height,
            &mut transactions.iter(),
        );

        let state_hash = {
            let core_state_hash = BlockchainSchema::read(&fork).core_state_hash();
            let app_state_hash = self.app.state_hash(&fork);

            {
                let mut schema = BlockchainSchema::write(&mut fork);
                for (i, hash) in core_state_hash.into_iter().enumerate() {
                    let key = service_table_unique_key(0, i);
                    schema.insert_state_hash(&key, &hash);
                }
                for (i, hash) in app_state_hash.into_iter().enumerate() {
                    let key = service_table_unique_key(1, i);
                    schema.insert_state_hash(&key, &hash);
                }
            };
            BlockchainSchema::read(&fork).summary_state_hash()
        };

        let prev_block_hash = BlockchainSchema::read(&fork).last_block_hash();
        // Create block
        let block = Block::new(
            SCHEMA_MAJOR_VERSION,
            proposer_id,
            height,
            transactions.len() as u32,
            prev_block_hash,
            tx_hash,
            state_hash,
        );
        trace!("execute block = {:?}", block);
        // Update height
        let block_hash = BlockchainSchema::write(&mut fork).insert_block(block);
        (block_hash, fork.into_patch())
    }
}

/// Helper function to map tuple (`u16`, `u16`) of service table coordinates
/// to 32 byte value for use as `MerklePatriciaTable` key (it currently
/// supports only fixed size keys). `hash` function is used to distribute
/// keys uniformly (compared to padding).
/// # Arguments
///
/// * `service_id` - `service_id` as returned by instance of type of
/// `Service` trait
/// * `table_idx` - index of service table in `Vec`, returned by
/// `state_hash` method of instance of type of `Service` trait
// also, it was the first idea around, to use `hash`
pub fn service_table_unique_key(service_id: u16, table_idx: usize) -> Hash {
    debug_assert!(table_idx <= u16::max_value() as usize);
    const SIZE: usize = 2;
    assert_eq!(SIZE, ::std::mem::size_of::<u16>());
    let mut buff = [0; 2 * SIZE];
    LittleEndian::write_u16(&mut buff[0..SIZE], service_id);
    LittleEndian::write_u16(&mut buff[SIZE..2 * SIZE], table_idx as u16);
    ::crypto::hash(&buff)
}
