use std::fmt::Write;
use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, CommonRpcStringsRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

/// 운영되는 블록체인 노드의 버전을 제공하는 RPC
/// # Example
/// * "Biiot/v0.1.0/windows/rust1.52"
pub struct Web3ClientVersion(Box<CommonRpcEmptyRequest>);

// impl From<RpcEmptyRequest> for Web3ClientVersion {
//     fn from(request: RpcEmptyRequest) -> Self {
//         Self { 0: request }
//     }
// }

// impl From<RpcEmptyRequestWithUuid> for Web3ClientVersion {
//     fn from(request: RpcEmptyRequestWithUuid) -> Self {
//         Self { 0: request }
//     }
// }

impl From<Box<CommonRpcEmptyRequest>> for Web3ClientVersion {
    fn from(request: Box<CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for Web3ClientVersion {
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request = RpcEmptyRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method());
                serde_json::to_string::<RpcEmptyRequestWithUuid>(&request).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request = RpcEmptyRequest::new(&id, RPC_VERSION, self.0.method());
                serde_json::to_string::<RpcEmptyRequest>(&request).unwrap()
            }
        }
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let result = "Biiot/v0.1.0/windows/rust1.52";
        return match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), result);
                serde_json::to_string(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &result);
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}

pub struct Web3Sha3(Box<CommonRpcStringsRequest>);

// impl From<RpcStringsRequest> for Web3Sha3 {
//     fn from(request: RpcStringsRequest) -> Self {
//         Self { 0: request }
//     }
// }

impl From<Box<CommonRpcStringsRequest>> for Web3Sha3 {
    fn from(request: Box<CommonRpcStringsRequest>) -> Self {
        return Self { 0: request }
    }
}

impl RpcCall for Web3Sha3 {
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request = RpcStringsRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string::<RpcStringsRequestWithUuid>(&request).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request = RpcStringsRequest::new(&id, RPC_VERSION, self.0.method(), self.0.params());
                serde_json::to_string::<RpcStringsRequest>(&request).unwrap()
            }
        }
        // serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap()
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        let data = self.0.params().get(0).unwrap().as_str().unwrap().as_bytes();
        let hex_data = hex::decode(data).unwrap();

        let u8a32h = crypto::hash::keccak256(hex_data.as_slice());
        let digest = hex::encode(u8a32h);
        let mut result = "0x".to_string();
        write!(&mut result, "{}", digest);

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
    }
}