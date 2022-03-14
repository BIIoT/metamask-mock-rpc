use std::sync::Mutex;
use hub_channel::channel::Channel;

pub mod web3;
pub mod net_version;
pub mod eth_protocol_version;
pub mod eth_gas_price;
pub mod eth_block_number;
pub mod eth_get_balance;
pub mod eth_get_transaction_count;
pub mod eth_send_raw_transaction;
pub mod eth_estimate_gas;
pub mod eth_get_block_by_hash;
pub mod eth_chain_id;
pub mod eth_get_block_by_number;
pub mod eth_call;
pub mod debug;

/// RPC 메서드들에 대한 공통 특성
pub trait RpcCall {
    /// 노드가 해당 RPC를 호출할 때 사용하는 메서드
    fn call(&self) -> String;
    /// 노드가 해당 RPC를 요청받을 때 사용하는 메서드
    fn receive(&self, ch: &Mutex<Channel>) -> String;
}