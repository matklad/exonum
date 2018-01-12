extern crate iron;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate exonum;

use iron::Handler;

use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum::helpers::fabric::{ConfigServiceFactory, Context};
use exonum::helpers::fabric::keys;
use exonum::crypto::{PublicKey, Hash};
use exonum::blockchain::{ApiContext, Transaction, Service, ServiceContext};
use exonum::messages::RawTransaction;
use exonum::storage::{Fork, Snapshot};
use exonum::encoding::Error as EncodingError;


pub const ID: u16 = 128;
pub const NAME: &str = "collector";


pub struct CollectorServiceFactory {}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    owner_key: PublicKey,
}

impl ConfigServiceFactory for CollectorServiceFactory {
    type Config = ServiceConfig;
    const SERVICE_NAME: &'static str = NAME;

    fn make_service(&mut self, context: &Context, config: ServiceConfig) -> Box<Service> {
        let node_config = context.get(keys::NODE_CONFIG).unwrap();
        let service_config: ServiceConfig = node_config.services_configs[NAME]
            .clone()
            .try_into()
            .unwrap();

        Box::new(CollectorService::new(config))
    }
}

pub struct CollectorService {
    config: ServiceConfig,
}

impl CollectorService {
    fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
}

impl Service for CollectorService {
    fn service_name(&self) -> &'static str {
        NAME
    }

    fn service_id(&self) -> u16 {
        ID
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        Vec::new()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, EncodingError> {
        unimplemented!()
    }

    fn initialize(&self, fork: &mut Fork) -> json::Value {
        unimplemented!()
    }

    fn handle_commit(&self, context: &ServiceContext) {
        unimplemented!()
    }

    fn private_api_handler(&self, _context: &ApiContext) -> Option<Box<Handler>> {
        unimplemented!();
    }

    fn public_api_handler(&self, context: &ApiContext) -> Option<Box<Handler>> {
        unimplemented!()
    }
}



fn main() {
    helpers::init_logger().unwrap();
    let node = NodeBuilder::new().with_service(CollectorServiceFactory {});
    node.run();
}
