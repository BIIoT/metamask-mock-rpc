use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::block::PseudoBlock;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcStringsRequest, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::response::{RpcBlockResponse, RpcBlockResponseWithUuid, RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthGetBlockByNumber(Box<dyn CommonRpcStringsRequest>);

impl From<Box<dyn CommonRpcStringsRequest>> for EthGetBlockByNumber {
    fn from(request: Box<dyn CommonRpcStringsRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthGetBlockByNumber {
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
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let result = PseudoBlock::default();
        return match self.0.is_uuid() {
            true => {
                let res = RpcBlockResponseWithUuid::new(self.0.str_id().as_str(), &result);
                serde_json::to_string(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcBlockResponse::new(&id, &result);
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}