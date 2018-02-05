use std::panic;

mod block;
pub(crate) use self::block::Block;
mod height;
mod schema;
pub(crate) use self::schema::{BlockchainSchema, SCHEMA_MAJOR_VERSION};

pub use self::height::Height;
use storage;
use storage::db::Fork;
use transaction::Tx;
use crypto::Hash;


pub(crate) fn execute_transactions(
    fork: &mut Fork,
    height: Height,
    transactions: &mut Iterator<Item=&Tx>,
) -> Hash
{
    // Save & execute transactions
    for (index, tx) in transactions.enumerate() {
        let hash = tx.hash();
        fork.checkpoint();
        let r = panic::catch_unwind(panic::AssertUnwindSafe(|| tx.execute(fork)));

        match r {
            Ok(..) => fork.commit(),
            Err(err) => {
                if err.is::<storage::Error>() {
                    // Continue panic unwind if the reason is StorageError
                    panic::resume_unwind(err);
                }
                fork.rollback();
                error!("{:?} transaction execution failed: {:?}", hash, err);
            }
        }

        {
            let mut schema = BlockchainSchema::write(&mut *fork);
            schema.insert_transaction(height, index as u64, &tx);
        };
    }
    let schema = BlockchainSchema::read(& *fork);
    schema.block_txs_hash(height)
}


impl Tx {
    fn bytes(&self) -> Vec<u8> {
        let mut r = Vec::new();
        self.store_tx(&mut r);
        r
    }
}
