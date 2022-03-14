use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthGasPrice(Box<CommonRpcEmptyRequest>);

impl From<Box<CommonRpcEmptyRequest>> for EthGasPrice {
    fn from(request: Box<CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthGasPrice {
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
        // serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap()
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let price = "0x00".to_string();
        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), &price);
                serde_json::to_string::<RpcStringResponseWithUuid>(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &price);
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, &price);
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}