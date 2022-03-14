use std::io::Write;
use std::sync::Mutex;
use basic_http::request::HttpRequest;
use basic_http::response::HttpResponse;
use basic_http::status::HttpStatusCode;
use hub_channel::channel::Channel;
use log::{info, warn};
use serde_json::{Error, Value};
use crate::consts::RPC_VERSION;
use crate::methods::eth_block_number::EthBlockNumber;
use crate::methods::eth_call::EthCall;
use crate::methods::eth_chain_id::EthChainId;
use crate::methods::eth_estimate_gas::EthEstimateGas;
use crate::methods::eth_gas_price::EthGasPrice;
use crate::methods::eth_get_balance::EthGetBalance;
use crate::methods::eth_get_block_by_number::EthGetBlockByNumber;
use crate::methods::eth_get_transaction_count::EthGetTransactionCount;
use crate::methods::eth_protocol_version::EthProtocolVersion;
use crate::methods::eth_send_raw_transaction::EthSendRawTransaction;
use crate::methods::net_version::NetVersion;
use crate::methods::RpcCall;
use crate::methods::web3::{Web3ClientVersion, Web3Sha3};
use crate::request::{CommonRpcEmptyRequest, CommonRpcStringsRequest, new_common_rpc_empty_request, new_common_rpc_object_request, new_common_rpc_strings_request, RequestIdtype, RpcEmptyRequest, RpcEmptyRequestWithUuid, RpcStringsRequest, RpcStringsRequestWithUuid};
use crate::utils::fileutil::load_file;
use crate::utils::timeutil::str_utc_time_now;

pub fn rpc_handler(request: HttpRequest, mut response: HttpResponse, ch: &Mutex<Channel>) {
    let data = request.body().data();
    if data.len() == 0 {
        warn!("[JSON_RPC] data length is zero. Are you trying to connect via Firefox Metamask?");
        response.set_code(HttpStatusCode::BadRequest);
        response.send(); // 응답을 주지 않으면 반항심 때문인지 계속 보낸다..
        return;
    }

    let a: Result<Value, serde_json::Error> = serde_json::from_str(data);
    match a {
        Ok(_) => { /* 정상적인 경우에는 실행할 내용이 없다. */ }
        Err(_) => {
            warn!("[JSON_RPC] EOF while parsing a string");
            return;
        }
    }
    let rpc_object: Value = serde_json::from_str(data).unwrap();
    let opt_rpc_method = rpc_object.get("method");
    let opt_rpc_id = rpc_object.get("id");
    let opt_rpc_params = rpc_object.get("params");

    // 셋 중 하나라도 없다면 정상적인 RPC 콜이 아니다.
    if opt_rpc_method.is_none() && opt_rpc_id.is_none() && opt_rpc_params.is_none() {
        warn!("[JSON_RPC] broken message received");
        return;
    }

    let mut rpc_id_type: RequestIdtype;
    match opt_rpc_id.unwrap().as_u64() {
        None => rpc_id_type = RequestIdtype::Uuid,
        Some(u64_rpc_id) => rpc_id_type = RequestIdtype::U64,
    }
    let rpc_method = opt_rpc_method.unwrap().as_str().unwrap();
    let rpc_params = opt_rpc_params.unwrap().as_array().unwrap();

    info!("[JSON_RPC] ip={} method='{}'", request.peer_addr, rpc_method);

    match rpc_method {
        "web3_clientVersion" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            let data = Web3ClientVersion::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "web3_sha3" => {
            const METHOD: &str = "web3_sha3";
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            // let rpc_request = RpcStringsRequest::new(rpc_id, RPC_VERSION, "web3_sha3", rpc_params);
            let data = Web3Sha3::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "net_version" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            // let rpc_request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "net_version");
            let data = NetVersion::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "net_peerCount" => { /* unsupported method */ }
        "net_listening" => { /* unsupported method */}
        "eth_protocolVersion" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            // let rpc_request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "eth_protocolVersion");
            let data = EthProtocolVersion::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_syncing" => { /* unsupported method */ }
        "eth_coinbase" => { /* unsupported method */ }
        "eth_mining" => { /* unsupported method */ }
        "eth_hashrate" => { /* unsupported method */ }
        "eth_gasPrice" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            // let rpc_request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "eth_gasPrice");
            let data = EthGasPrice::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_accounts" => {
            let channel = ch.lock().unwrap();
            // std::mem::drop(channel);
        }
        "eth_blockNumber" => {
            // let request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "eth_blockNumber");
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, "eth_blockNumber");
            let data = EthBlockNumber::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_getBalance" => {
            let rpc_request =
                new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            // let rpc_request = RpcStringsRequest::new(rpc_id, RPC_VERSION, "eth_getBalance", rpc_params);
            let data = EthGetBalance::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_getStorageAt" => {}
        "eth_getTransactionCount" => {
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            let data = EthGetTransactionCount::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_getBlockTransactionCountByHash" => {}
        "eth_getBlockTransactionCountByNumber" => {}
        "eth_getUncleCountByBlockHash" => {}
        "eth_getUncleCountByBlockNumber" => {}
        "eth_getCode" => {}
        "eth_sign" => {}
        "eth_signTransaction" => {}
        "eth_sendTransaction" => {}
        "eth_sendRawTransaction" => {
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            let mut str_params: &str;
            for param in rpc_request.params().iter() {
                let compatible_filename = format!("./{}.txt", str_utc_time_now().replace(":", "_"));
                info!("filename: {}", compatible_filename);
                let mut f = load_file(compatible_filename.as_str()).unwrap();
                let str_param = format!("{}", param);
                f.write(str_param.as_bytes());
            }
            let data = EthSendRawTransaction::from(rpc_request).receive(ch);
            println!(r#"{}"#, data.as_str());
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_call" => {
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            let data = EthCall::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
            // println!("{}", rpc_request.params().len());
            // for param in rpc_request.params().iter() {
            //     println!("{}", param);
            // }
        }
        "eth_estimateGas" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            // let rpc_request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "eth_estimateGas");
            let data = EthEstimateGas::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_getBlockByHash" => {}
        "eth_getBlockByNumber" => {
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            let data = EthGetBlockByNumber::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        "eth_getTransactionByHash" => {}
        "eth_getTransactionByBlockHashAndIndex" => {}
        "eth_getTransactionByBlockNumberAndIndex" => {}
        "eth_getTransactionReceipt" => {}
        "eth_getUncleByBlockHashAndIndex" => {}
        "eth_getUncleByBlockNumberAndIndex" => {}
        "eth_getCompilers" => {}
        "eth_compileLLL" => {}
        "eth_compileSolidity" => {}
        "eth_compileSerpent" => {}
        "eth_newFilter" => {}
        "eth_newBlockFilter" => {}
        "eth_newPendingTransactionFilter" => {}
        "eth_uninstallFilter" => {}
        "eth_getFilterChanges" => {}
        "eth_getFilterLogs" => {}
        "eth_getLogs" => {}
        "eth_getWork" => {}
        "eth_submitWork" => {}
        "eth_submitHashrate" => {}
        "db_putString" => {}
        "db_getString" => {}
        "db_putHex" => {}
        "db_getHex" => {}
        "shh_post" => {}
        "shh_version" => {}
        "shh_newIdentity" => {}
        "shh_hasIdentity" => {}
        "shh_newGroup" => {}
        "shh_addToGroup" => {}
        "shh_newFilter" => {}
        "shh_uninstallFilter" => {}
        "shh_getFilterChanges" => {}
        "shh_getMessages" => {}
        "eth_chainId" => {
            let rpc_request = new_common_rpc_empty_request(rpc_id_type, opt_rpc_id, rpc_method);
            let data = EthChainId::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        // "eth_chainId" => {
        //     const METHOD: &str = "eth_chainId";
        //     let rpc_request: Box<dyn CommonRpcEmptyRequest>;
        //     match rpc_id_type {
        //         RequestIdtype::Uuid => {
        //             let rpc_id = opt_rpc_id.unwrap().as_str().unwrap();
        //             rpc_request = Box::new(RpcEmptyRequestWithUuid::new(rpc_id, RPC_VERSION, METHOD));
        //         }
        //         RequestIdtype::U64 => {
        //             let rpc_id = opt_rpc_id.unwrap().as_u64().unwrap();
        //             rpc_request = Box::new(RpcEmptyRequest::new(&rpc_id, RPC_VERSION, METHOD));
        //         }
        //     }
        //     // let rpc_request = RpcEmptyRequest::new(rpc_id, RPC_VERSION, "net_version");
        //     let data = EthChainId::from(rpc_request).receive(ch);
        //     response.set_code(HttpStatusCode::Ok);
        //     response.set_data(data.as_str());
        // }
        "debugRawTransaction" => {
            let rpc_request = new_common_rpc_strings_request(rpc_id_type, opt_rpc_id, rpc_method, rpc_params);
            let data = crate::methods::debug::debug_raw_transaction::DebugRawTransaction::from(rpc_request).receive(ch);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        &_ => {
            println!("[JSON_RPC] could not handle the method '{}'", rpc_method);
            response.set_code(HttpStatusCode::NotFound);
            response.set_data("");
        }
    }
    match response.get_data().len() {
        0 => {
            warn!("[JSON_RPC] ip={} could not handled '{}' due to internal error", request.peer_addr, rpc_method);
        }
        _ => {}
    }
    response.send();
}