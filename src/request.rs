use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use crate::consts::RPC_VERSION;

pub enum RequestIdtype {
    Uuid,
    U64,
}

pub trait RawRequestParams {
    fn raw_params(&self) -> Vec<Value>;
}

pub trait CommonRpcEmptyRequest {
    fn is_uuid(&self) -> bool;
    fn method(&self) -> &str;
    fn str_id(&self) -> String;
}

pub trait CommonRpcStringsRequest: RawRequestParams {
    fn is_uuid(&self) -> bool;
    fn method(&self) -> &str;
    fn str_id(&self) -> String;
    fn params(&self) -> &Vec<Value>;
}

pub trait CommonRpcIntegerRequest: RawRequestParams {
    fn is_uuid(&self) -> bool;
    fn method(&self) -> &str;
    fn str_id(&self) -> String;
}

pub trait CommonRpcObjectRequest: RawRequestParams {
    fn is_uuid(&self) -> bool;
    fn method(&self) -> &str;
    fn str_id(&self) -> String;
    fn params(&self) -> HashMap<String, String>;
}

/// Empty Request with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcEmptyRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    params: Vec<u8>,
}

impl RpcEmptyRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: vec![],
        }
    }
}

impl CommonRpcEmptyRequest for RpcEmptyRequest {
    fn is_uuid(&self) -> bool {
        false
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.to_string()
    }
}

/// Empty Request with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcEmptyRequestWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    params: Vec<u8>,
}

impl RpcEmptyRequestWithUuid {
    pub fn new(id: &str, jsonrpc: &str, method: &str) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: vec![],
        }
    }
}

impl CommonRpcEmptyRequest for RpcEmptyRequestWithUuid {
    fn is_uuid(&self) -> bool {
        true
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.as_str().to_string()
    }
}

/// String Array Request with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcStringsRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcStringsRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.to_vec(),
        }
    }
}

impl RawRequestParams for RpcStringsRequest {
    fn raw_params(&self) -> Vec<Value> {
        self.params.clone()
    }
}

impl CommonRpcStringsRequest for RpcStringsRequest {
    fn is_uuid(&self) -> bool {
        false
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.to_string()
    }

    fn params(&self) -> &Vec<Value> {
        &self.params
    }
}

/// String Array Request with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcStringsRequestWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcStringsRequestWithUuid {
    pub fn new(id: &str, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.to_vec(),
        }
    }
}

impl RawRequestParams for RpcStringsRequestWithUuid {
    fn raw_params(&self) -> Vec<Value> {
        self.params.clone()
    }
}

impl CommonRpcStringsRequest for RpcStringsRequestWithUuid {
    fn is_uuid(&self) -> bool {
        true
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.as_str().to_string()
    }

    fn params(&self) -> &Vec<Value> {
        &self.params
    }
}

/// Integer Request with u64 ID
#[derive(Serialize, Deserialize)]
pub struct RpcIntegerRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcIntegerRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.to_vec(),
        }
    }
}

impl RawRequestParams for RpcIntegerRequest {
    fn raw_params(&self) -> Vec<Value> {
        self.params.to_vec()
    }
}

impl CommonRpcIntegerRequest for RpcIntegerRequest {
    fn is_uuid(&self) -> bool {
        false
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.to_string()
    }
}

/// Integer Request with UUID
#[derive(Serialize, Deserialize)]
pub struct RpcIntegerRequestWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcIntegerRequestWithUuid {
    pub fn new(id: &str, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.to_vec(),
        }
    }
}

impl RawRequestParams for RpcIntegerRequestWithUuid {
    fn raw_params(&self) -> Vec<Value> {
        self.params.to_vec()
    }
}

impl CommonRpcIntegerRequest for RpcIntegerRequestWithUuid {
    fn is_uuid(&self) -> bool {
        true
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.as_str().to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcObjectRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcObjectRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.clone(),
        }
    }
}

impl RawRequestParams for RpcObjectRequest {
    fn raw_params(&self) -> Vec<Value> {
        self.params.to_vec()
    }
}

impl CommonRpcObjectRequest for RpcObjectRequest {
    fn is_uuid(&self) -> bool {
        false
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.to_string()
    }

    fn params(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        let map_list = self.params.get(0).unwrap().as_object().unwrap();
        map_list.iter().for_each(|b| {
            map.insert(b.0.to_string(), b.1.as_str().unwrap().to_string());
        });
        return map;
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcObjectRequestWithUuid {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

impl RpcObjectRequestWithUuid {
    pub fn new(id: &str, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: params.clone(),
        }
    }
}

impl CommonRpcObjectRequest for RpcObjectRequestWithUuid {
    fn is_uuid(&self) -> bool {
        true
    }

    fn method(&self) -> &str {
        self.method.as_str()
    }

    fn str_id(&self) -> String {
        self.id.as_str().to_string()
    }

    fn params(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        let map_list = self.params.get(0).unwrap().as_object().unwrap();
        map_list.iter().for_each(|b| {
            map.insert(b.0.to_string(), b.1.as_str().unwrap().to_string());
        });
        return map;
    }
}

impl RawRequestParams for RpcObjectRequestWithUuid {
    fn raw_params(&self) -> Vec<Value> {
        self.params.clone()
    }
}

pub fn new_common_rpc_empty_request(rpc_id_type: RequestIdtype, opt_rpc_id: Option<&Value>, method: &str) -> Box<dyn CommonRpcEmptyRequest> {
    let result: Box<dyn CommonRpcEmptyRequest>;
    match rpc_id_type {
        RequestIdtype::Uuid => {
            let request = RpcEmptyRequestWithUuid::new(opt_rpc_id.unwrap().as_str().unwrap(), RPC_VERSION, method);
            result = Box::new(request);
        }
        RequestIdtype::U64 => {
            let id = opt_rpc_id.unwrap().as_u64().unwrap();
            let request = RpcEmptyRequest::new(&id, RPC_VERSION, method);
            result = Box::new(request);
        }
    }
    return result;
}

pub fn new_common_rpc_strings_request(rpc_id_type: RequestIdtype, opt_rpc_id: Option<&Value>, method: &str, params: &Vec<Value>) -> Box<dyn CommonRpcStringsRequest> {
    let result: Box<dyn CommonRpcStringsRequest>;
    match rpc_id_type {
        RequestIdtype::Uuid => {
            let rpc_id = opt_rpc_id.unwrap().as_str().unwrap();
            let request = RpcStringsRequestWithUuid::new(rpc_id, RPC_VERSION, method, params);
            result = Box::new(request);
        }
        RequestIdtype::U64 => {
            let id = opt_rpc_id.unwrap().as_u64().unwrap();
            let request = RpcStringsRequest::new(&id, RPC_VERSION, method, params);
            result = Box::new(request);
        }
    }
    return result;
}

pub fn new_common_rpc_object_request(rpc_id_type: RequestIdtype, opt_rpc_id: Option<&Value>, method: &str, params: &Vec<Value>) -> Box<dyn CommonRpcObjectRequest> {
    let result: Box<dyn CommonRpcObjectRequest>;
    match rpc_id_type {
        RequestIdtype::Uuid => {
            let rpc_id = opt_rpc_id.unwrap().as_str().unwrap();
            let request = RpcObjectRequestWithUuid::new(rpc_id, RPC_VERSION, method, params);
            result = Box::new(request);
        }
        RequestIdtype::U64 => {
            let id = opt_rpc_id.unwrap().as_u64().unwrap();
            let request = RpcObjectRequest::new(&id, RPC_VERSION, method, &params);
            result = Box::new(request);
        }
    }
    return result;
}