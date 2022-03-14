use ethereum_types::{Address, H256, U256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename="blockHash")]
    pub block_hash: H256,
    #[serde(rename="blockNumber")]
    pub block_number: u64,
    pub from: Address,
    pub gas: U256,
    #[serde(rename="gasPrice")]
    pub gas_price: U256,
    pub hash: H256,
    pub input: Vec<u8>,
    pub nonce: u64,
    pub to: Address,
    #[serde(rename="transactionIndex")]
    pub transaction_index: u64,
    pub value: U256,
    pub v: i32,
    pub r: H256,
    pub s: H256,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            block_hash: H256::zero(),
            block_number: 0,
            from: Address::zero(),
            gas: U256::zero(),
            gas_price: U256::zero(),
            hash: H256::zero(),
            input: vec![],
            nonce: 0,
            to: Address::zero(),
            transaction_index: 0,
            value: U256::zero(),
            v: 0,
            r: H256::zero(),
            s: H256::zero(),
        }
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Self {
        Self {
            block_hash: self.block_hash.clone(),
            block_number: self.block_number.clone(),
            from: self.from.clone(),
            gas: self.gas.clone(),
            gas_price: self.gas_price.clone(),
            hash: self.hash.clone(),
            input: self.input.to_vec(),
            nonce: self.nonce.clone(),
            to: self.to.clone(),
            transaction_index: self.transaction_index.clone(),
            value: self.value.clone(),
            v: self.v.clone(),
            r: self.r.clone(),
            s: self.s.clone(),
        }
    }
}