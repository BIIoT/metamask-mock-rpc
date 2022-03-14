use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcStringsRequest, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthGetBlockByHash(Box<dyn CommonRpcStringsRequest>);

impl From<Box<dyn CommonRpcStringsRequest>> for EthGetBlockByHash {
    fn from(request: Box<dyn CommonRpcStringsRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthGetBlockByHash {
    fn call(&self) -> String {
        match self.0.is_uuid() {
            true => {
                let request = RpcStringsRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string(&request).unwrap()
            }
            false => {
                let id = u64::from_str(&self.0.str_id().as_str()).unwrap();
                let request = RpcStringsRequest::new(&id, RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string(&request).unwrap()
            }
        }
        // serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap()
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let result = "0x0";

        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), &result);
                serde_json::to_string(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &result);
                serde_json::to_string(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, &result);
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}