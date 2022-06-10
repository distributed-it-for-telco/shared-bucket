use wasmbus_rpc::actor::prelude::*;
use shared_bucket::{Customers};
use wasmcloud_interface_logging::{info};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor)]
struct ServiceVendorsActor {}

// Implementation of trait methods : TODO


