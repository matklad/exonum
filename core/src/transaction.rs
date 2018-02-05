use std::cell::Cell;

use crypto::Hash;
use storage::db::Fork;

pub trait CoreTransaction: 'static {
    fn execute(&self, fork: &mut Fork);
    fn store_tx(&self, buff: &mut Vec<u8>);
    fn compute_hash(&self) -> Hash;
}

pub(crate) struct Tx {
    inner: Box<CoreTransaction>,
    hash_cache: Cell<Option<Hash>>,
}

impl Tx {
    pub fn execute(&self, fork: &mut Fork) {
        self.inner.execute(fork)
    }

    pub fn store_tx(&self, buff: &mut Vec<u8>) {
        self.inner.store_tx(buff)
    }

    pub fn hash(&self) -> Hash {
        match self.hash_cache.get() {
            Some(hash) => hash,
            None => {
                let hash = self.inner.compute_hash();
                self.hash_cache.set(Some(hash));
                hash
            }
        }
    }
}
