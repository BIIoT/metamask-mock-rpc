use std::str::FromStr;
use std::sync::Mutex;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcEmptyRequest, RpcEmptyRequest, RpcEmptyRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthBlockNumber(Box<CommonRpcEmptyRequest>);

impl From<Box<CommonRpcEmptyRequest>> for EthBlockNumber {
    fn from(request: Box<CommonRpcEmptyRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthBlockNumber {
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
        // serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap()
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        // let channel = ch.lock().unwrap();
        //
        // let mut result: String;
        // let eth_block_number = ledger_message::request::writer::LedgerRequestWriter::eth_block_number(&channel.endpoint);
        // channel.send(eth_block_number);
        // let eth_block_number_result = channel.receive_seconds_until(30);
        // match eth_block_number_result {
        //     Ok(eth_block_number_response) => {
        //         let h256_block_number = H256::from_slice(eth_block_number_response.data().as_slice());
        //         let u64_block_number = h256_block_number.to_low_u64_le();
        //         let hex_block_number = format!("0x{:x}", u64_block_number);
        //         let res = RpcStringResponse::new(self.0.id, hex_block_number.as_str());
        //         serde_json::to_string::<RpcStringResponse>(&res).unwrap()
        //     }
        //     Err(_) => {
        //         // Do not response any message (원래는 안 보내기로 했으나 테스트값을 전송하는 것으로 변경)
        //         let result = "0x0"; //"0x4b7";
        //         let res = RpcStringResponse::new(self.0.id, result);
        //         serde_json::to_string::<RpcStringResponse>(&res).unwrap()
        //     }
        // }
        let result = "0x0";
        match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), result);
                serde_json::to_string::<RpcStringResponseWithUuid>(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, result);
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, result);
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}