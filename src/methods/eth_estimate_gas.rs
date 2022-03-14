use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthEstimateGas(Box<dyn CommonRpcEmptyRequest>);

impl From<Box<dyn CommonRpcEmptyRequest>> for EthEstimateGas {
    fn from(request: Box<dyn CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthEstimateGas {
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
        const ZERO_GAS: &str = "0x0";

        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), &ZERO_GAS);
                serde_json::to_string(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &ZERO_GAS);
                serde_json::to_string(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, &result);
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}