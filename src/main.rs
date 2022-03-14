use std::sync::{Arc, Mutex};
use hub_channel::hub::ChannelHub;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

mod consts;
mod handler;
mod listener;
mod methods;
mod request;
mod response;
mod account;
mod logging;
mod signer;
mod block;
mod transaction;
mod raw_transaction;
pub mod utils;


fn main() {
    crate::logging::init();
    log::info!("Metamask sign checker is running now");
    let mut hub = ChannelHub::new();
    let rpc_container = Arc::new(Mutex::new(hub.new_container("rpc")));
    let rpc = crate::listener::RpcListener::new("0.0.0.0", 8545, rpc_container);
    rpc.run();
}
