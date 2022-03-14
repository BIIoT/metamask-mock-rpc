use std::fmt::format;
use std::str::FromStr;
use std::sync::Mutex;
use ethereum_types::Address;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcStringsRequest, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthGetTransactionCount(Box<dyn CommonRpcStringsRequest>);

impl From<Box<dyn CommonRpcStringsRequest>> for EthGetTransactionCount {
    fn from(request: Box<dyn CommonRpcStringsRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthGetTransactionCount {
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request = RpcStringsRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string(&request).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request = RpcStringsRequest::new(&id, RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string(&request).unwrap()
            }
        }
        // serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap()
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let str_address = self.0.params().get(0).unwrap().as_str().unwrap().split_at(2).1;
        let hex_address = hex::decode(str_address).unwrap();
        let address = Address::from_slice(hex_address.as_slice());
        let tx_count = format!("{:x}", 1u64);

        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), tx_count.as_str());
                serde_json::to_string::<RpcStringResponseWithUuid>(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, tx_count.as_str());
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, tx_count.as_str());
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}