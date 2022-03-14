use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::Mutex;
use ethereum_types::{Address, U256};
use hub_channel::channel::Channel;
use hub_channel::endpoint::ChannelEndpoint;
use hub_channel::message::ChannelMessage;
use rlp::{Encodable, RlpStream};
use crate::consts::RPC_VERSION;
use crate::methods::RpcCall;
use crate::request::{CommonRpcObjectRequest, CommonRpcStringsRequest, RawRequestParams, RpcObjectRequest, RpcObjectRequestWithUuid};

pub struct EthCallParams {
    pub from: Address,
    pub to: Address,
    pub gas: u64,
    pub gas_price: U256,
    pub value: U256,
    pub data: Vec<u8>
}

impl From<&HashMap<String, String>> for EthCallParams {
    fn from(params: &HashMap<String, String>) -> Self {
        let mut result = Self::default();
        params.iter().for_each(|kv| {
            match kv.0.as_str() {
                "from" => result.from = Address::from_str(kv.1.as_str()).unwrap(),
                "to" => result.to = Address::from_str(kv.1.as_str()).unwrap(),
                "gas" => result.gas = u64::from_str(kv.1.as_str()).unwrap(),
                "gas_price" => result.gas_price = U256::from_str(kv.1.as_str()).unwrap(),
                "value" => result.value = U256::from_str(kv.1.as_str()).unwrap(),
                "data" => {
                    let data = kv.1.as_str().to_string();
                    result.data = hex::decode(data.replace("0x", "")).unwrap()
                },
                &_ => {}
            }
        });
        return result;
    }
}

impl Encodable for EthCallParams {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6);
        s.append(&self.from);
        s.append(&self.to);
        s.append(&self.gas);
        s.append(&self.gas_price);
        s.append(&self.value);
        s.append(&self.data);
    }
}

impl Default for EthCallParams {
    fn default() -> Self {
        Self {
            from: Address::zero(),
            to: Address::zero(),
            gas: 0,
            gas_price: U256::zero(),
            value: U256::zero(),
            data: vec![],
        }
    }
}

pub struct EthCall(Box<dyn CommonRpcStringsRequest>);

impl From<Box<dyn CommonRpcStringsRequest>> for EthCall {
    fn from(request: Box<dyn CommonRpcStringsRequest>) -> Self {
        Self { 0: request }
    }
}

impl RpcCall for EthCall {
    fn call(&self) -> String {
        return match self.0.is_uuid() {
            true => {
                let request =
                    RpcObjectRequestWithUuid::new(self.0.str_id().as_str(), RPC_VERSION, self.0.method(), &self.0.raw_params());
                serde_json::to_string(&request).unwrap()
            }
            false => {
                let id = u64::from_str(self.0.str_id().as_str()).unwrap();
                let request =
                    RpcObjectRequest::new(&id, RPC_VERSION, self.0.method(), &self.0.raw_params());
                serde_json::to_string(&request).unwrap()
            }
        }
    }

    fn receive(&self, ch: &Mutex<Channel>) -> String {
        // let channel = ch.lock().unwrap();
        // let params = EthCallParams::from(&self.0.params());
        // let rlp_params = rlp::encode(&params);
        // let eth_call_request = ChannelMessage::new(channel.endpoint(), &ChannelEndpoint::default(), "get_eth_call", &rlp_params.to_vec(), "vm");
        // channel.send(eth_call_request);
        // let eth_call_response = channel.receive_seconds_until(5);
        // match eth_call_response {
        //     Ok(eth_call_result) => {
        //         let return_value = eth_call_result.data();
        //         let hex_encoded_return_value = hex::encode(return_value);
        //         println!("{}", &hex_encoded_return_value);
        //         return hex_encoded_return_value;
        //     }
        //     Err(_) => { return "".to_string(); }
        // }
        "".to_string()
    }
}