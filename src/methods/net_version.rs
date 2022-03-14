use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub const NET_VERSION: &str = "8504";

pub struct NetVersion(Box<dyn CommonRpcEmptyRequest>);

// impl From<RpcEmptyRequest> for NetVersion {
//     fn from(request: RpcEmptyRequest) -> Self {
//         Self { 0: request }
//     }
// }

impl From<Box<dyn CommonRpcEmptyRequest>> for NetVersion {
    fn from(request: Box<dyn CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for NetVersion {
    // fn call(&self) -> String {
    //     serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap()
    // }
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request = RpcEmptyRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method());
                serde_json::to_string::<RpcEmptyRequestWithUuid>(&request).unwrap()
            },
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request = RpcEmptyRequest::new(&id, RPC_VERSION, self.0.method());
                serde_json::to_string::<RpcEmptyRequest>(&request).unwrap()
            },
        };
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), &NET_VERSION.to_string());
                serde_json::to_string::<RpcStringResponseWithUuid>(&res).unwrap()
            }
            false => {
                let id = u64::from_str(&self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &NET_VERSION.to_string());
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, &NET_VERSION.to_string());
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}