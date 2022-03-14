use std::str::FromStr;
use std::sync::Mutex;
use crypto::hash::keccak256;
use hub_channel::channel::Channel;
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::raw_transaction::{MetamaskRawTransaction, RawTransaction};
use crate::request::{CommonRpcStringsRequest, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::response::{RpcStringResponse, RpcStringResponseWithUuid};

pub struct EthSendRawTransaction(Box<dyn CommonRpcStringsRequest>);

impl From<Box<dyn CommonRpcStringsRequest>> for EthSendRawTransaction {
    fn from(request: Box<dyn CommonRpcStringsRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthSendRawTransaction {
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
        let str_raw_tx = self.0.params().get(0).unwrap().as_str().unwrap().split_at(2).1;
        let mut result = "0x".to_string();

        let raw_tx = hex::decode(str_raw_tx).unwrap();
        let tx_hash = keccak256(raw_tx.as_slice());
        let str_tx_hash = hex::encode(tx_hash);
        result.push_str(str_tx_hash.as_str());

        let mrtx = rlp::decode::<MetamaskRawTransaction>(&raw_tx).unwrap();
        let rtx = RawTransaction::from(mrtx);
        // crate::signer::Eip155Signer::sign(rtx.nonce, )
        // crypto::ecdsa::public_key::PublicKey::from_signature(rtx.v.clone() as i32, )

        match self.0.is_uuid() {
            true => {
                let res = RpcStringResponseWithUuid::new(self.0.str_id().as_str(), &result);
                serde_json::to_string::<RpcStringResponseWithUuid>(&res).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let res = RpcStringResponse::new(&id, &result);
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        }
        // let res = RpcStringResponse::new(self.0.id, &result);
        // serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}