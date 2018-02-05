use crypto::Hash;
use storage::db::Fork;

use super::Height;
use transaction::Tx;
use storage::db::Snapshot;
use blockchain::Block;

pub(crate) const SCHEMA_MAJOR_VERSION: u16 = 0;

pub(crate) struct BlockchainSchema<V> {
    view: V
}

impl<'f> BlockchainSchema<&'f mut Fork> {
    pub fn write(fork: &'f mut Fork) -> Self {
        BlockchainSchema { view: fork }
    }

    pub fn insert_transaction(
        &mut self,
        _height: Height,
        _index: u64,
        _tx: &Tx,
    ) {
//        schema.transactions_mut().put(hash, tx.raw().clone());
//        schema.block_txs_mut(height).push(*hash);
//        let location = TxLocation::new(height, index as u64);
//        schema.tx_location_by_tx_hash_mut().put(hash, location);
    }

    pub fn insert_state_hash(
        &mut self,
        _key: &Hash,
        _value: &Hash,
    ) {

    }

    pub fn insert_block(&mut self, block: Block) -> Hash {
        let hash = block.hash();
//        schema.block_hashes_by_height_mut().push(block_hash);
        // Save block
//        schema.blocks_mut().put(&block_hash, block);
        hash
    }
}

impl<'f> BlockchainSchema<&'f Snapshot> {
    pub fn read(view: &'f Snapshot) -> Self {
        BlockchainSchema { view }
    }

    pub fn block_txs_hash(
        &self,
        height: Height,
    ) -> Hash {
//        schema.block_txs(height).root_hash();
        unimplemented!()
    }

    pub fn core_state_hash(&self) -> Vec<Hash> {
        Vec::new()
    }

    /// Returns the hash of latest committed block.
    ///
    /// # Panics
    ///
    /// - If the genesis block was not committed.
    pub fn last_block_hash(&self) -> Hash {
        unimplemented!()
    }

    pub fn summary_state_hash(&self) -> Hash {
        unimplemented!()
    }
}
