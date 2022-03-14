use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::panic::resume_unwind;
use std::str::FromStr;
use crate::block::PseudoBlock;
use crate::consts::RPC_VERSION;
use crate::request::CommonRpcStringsRequest;

/// String Response with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcStringResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: String,
}

impl RpcStringResponse {
    pub fn new(id: &u64, result: &str) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.to_string(),
        }
    }
}

/// String Response with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcStringResponseWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub result: String,
}

impl RpcStringResponseWithUuid {
    pub fn new(id: &str, result: &str) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcBlockResponseWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub result: PseudoBlock,
}

impl RpcBlockResponseWithUuid {
    pub fn new(id: &str, result: &PseudoBlock) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcBlockResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: PseudoBlock,
}

impl RpcBlockResponse {
    pub fn new(id: &u64, result: &PseudoBlock) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

/// Boolean Response with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcBoolResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: bool,
}

impl RpcBoolResponse {
    pub fn new(id: &u64, result: bool) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

/// Boolean Response with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcBoolResponseWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub result: bool,
}

impl RpcBoolResponseWithUuid {
    pub fn new(id: &str, result: bool) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

/// String Array Response with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcStringArrayResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: Vec<String>,
}

impl RpcStringArrayResponse {
    pub fn new(id: &u64, result: &Vec<String>) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.to_vec(),
        }
    }
}

/// String Array Response with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcStringArrayResponseWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub result: Vec<String>,
}

impl RpcStringArrayResponseWithUuid {
    pub fn new(id: &str, result: &Vec<String>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.to_vec(),
        }
    }
}

/// Map type Response with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcMapResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: HashMap<String, String>,
}

impl RpcMapResponse {
    pub fn new(id: &u64, result: &HashMap<String, String>) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

/// Map type Response with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcMapResponseWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub result: HashMap<String, String>,
}

impl RpcMapResponseWithUuid {
    pub fn new(id: &str, result: &HashMap<String, String>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: RPC_VERSION.to_string(),
            result: result.clone(),
        }
    }
}

pub fn new_json_rpc_string_response(request: &Box<dyn CommonRpcStringsRequest>, data: &str) -> String {
    return match request.is_uuid() {
        true => {
            let res = RpcStringResponseWithUuid::new(request.str_id().as_str(), data);
            serde_json::to_string(&res).unwrap()
        }
        false => {
            let id = u64::from_str(request.str_id().as_str()).unwrap();
            let res = RpcStringResponse::new(&id, data);
            serde_json::to_string(&res).unwrap()
        }
    }
}