extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate byteorder;

use storage::db::{Database, Fork, Snapshot};
use crypto::Hash;
pub use transaction::CoreTransaction;

mod crypto;
mod consensus;
mod blockchain;
mod storage;
mod core;
mod transaction;

pub fn spawn(
    db: Box<Database>,
    app: Box<App>,
) -> ExonumCoreApi {
    let _core = core::ExonumCore { db, app };

    ExonumCoreApi
}

pub trait App {
    fn load_tx(&self, raw: &[u8]) -> Box<CoreTransaction>;
    fn state_hash(&self, snap: &Snapshot) -> Vec<Hash>;
}

#[derive(Clone)]
pub struct ExonumCoreApi;

impl ExonumCoreApi {
    fn submit_transaction(&self, _tx: Box<CoreTransaction>) {}
}
