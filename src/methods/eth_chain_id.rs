use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub const CHAIN_ID: &str = "0x54"; // 84

pub struct EthChainId(Box<dyn CommonRpcEmptyRequest>);

impl From<Box<dyn CommonRpcEmptyRequest>> for EthChainId {
    fn from(request: Box<dyn CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthChainId {
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request = RpcEmptyRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method());
                serde_json::to_string(&request).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request = RpcEmptyRequest::new(&id, RPC_VERSION, self.0.method());
                serde_json::to_string(&request).unwrap()
            }
        }
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), CHAIN_ID);
                serde_json::to_string(&res).unwrap()
            }
            false => {
                let id = u64::from_str(&self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, CHAIN_ID);
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}