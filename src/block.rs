use ethereum_types::{Address, H256};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;


/// Ethereum Block structure
/// Web3와 블록 데이터를 주고 받을 때 사용한다.
/// # [Fields]
/// - `number: u64`                     - the block number. null when its pending block.
/// - `hash: H256`                      - hash of the block. null when its pending block.
/// - `parent_hash: H256`               - hash of the parent block.
/// - `nonce: [u8;8]`                   - hash of the generated proof-of-work. null when its pending block.
/// - `sha3_uncles: H256`               - SHA3 of the uncles data in the block.
/// - `logs_bloom: [u8;256]`            - the bloom filter for the logs of the block. null when its pending block.
/// - `transactions_root: H256`         - the root of the transaction trie of the block.
/// - `state_root: H256`                - the root of the final state trie of the block.
/// - `receipts_root: H256`              - the root of the receipts trie of the block.
/// - `miner: Address`                  - the address of the beneficiary to whom the mining rewards were given.
/// - `difficulty: u64`                 - integer of the difficulty for this block.
/// - `total_difficulty: u64`           - integer of the total difficulty of the chain until this block.
/// - `extra_data: Vec<u8>`             - the "extra data" field of this block.
/// - `size: u64`                       - integer the size of this block in bytes.
/// - `gas_limit: u64`                  - the maximum gas allowed in this block.
/// - `gas_used: u64`                   - the total used gas by all transactions in this block.
/// - `timestamp: u64`                  - the unix timestamp for when the block was collated.
/// - `transactions: Vec<Transaction>`  - Array of tx objects, or 32 Bytes tx hashes depending on the last given parameter.
/// - `uncles: Vec<H256>`               - Array of uncle hashes.
#[derive(Serialize, Deserialize)]
pub struct PseudoBlock {
    pub number: u64,
    pub hash: H256,
    #[serde(rename="parentHash")]
    pub parent_hash: H256,
    pub nonce: u64, //[u8; 8],
    #[serde(rename="sha3Uncles")]
    pub sha3_uncles: Vec<H256>,
    #[serde(rename="logsBloom")]
    pub logs_bloom: String,
    #[serde(rename="transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename="stateRoot")]
    pub state_root: H256,
    #[serde(rename="receiptsRoot")]
    pub receipts_root: H256,
    pub miner: Address,
    pub difficulty: u64,
    #[serde(rename="totalDifficulty")]
    pub total_difficulty: u64,
    #[serde(rename="extraData")]
    pub extra_data: Vec<u8>,
    pub size: u64,
    #[serde(rename="gasLimit")]
    pub gas_limit: u64,
    #[serde(rename="gasUsed")]
    pub gas_used: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    //
    pub uncles: Vec<H256>,                  // always zero-vector
}

impl Default for PseudoBlock {
    fn default() -> Self {
        Self {
            number: 0,
            hash: H256::zero(),
            parent_hash: H256::zero(),
            nonce: 0,
            sha3_uncles: vec![],
            logs_bloom: hex::encode([0u8;256].to_vec()),
            transactions_root: H256::zero(),
            state_root: H256::zero(),
            receipts_root: H256::zero(),
            miner: Address::zero(),
            difficulty: 0,
            total_difficulty: 0,
            extra_data: vec![],
            size: 0,
            gas_limit: 0,
            gas_used: 0,
            timestamp: 0,
            transactions: vec![],
            uncles: vec![],
        }
    }
}

impl Clone for PseudoBlock {
    fn clone(&self) -> Self {
        Self {
            number: self.number.clone(),
            hash: self.hash.clone(),
            parent_hash: self.parent_hash.clone(),
            nonce: self.nonce.clone(),
            sha3_uncles: self.sha3_uncles.to_vec(),
            logs_bloom: self.logs_bloom.clone(),
            transactions_root: self.transactions_root.clone(),
            state_root: self.state_root.clone(),
            receipts_root: self.receipts_root.clone(),
            miner: self.miner.clone(),
            difficulty: self.difficulty.clone(),
            total_difficulty: self.total_difficulty.clone(),
            extra_data: self.extra_data.to_vec(),
            size: self.size.clone(),
            gas_limit: self.gas_limit.clone(),
            gas_used: self.gas_used.clone(),
            timestamp: self.timestamp.clone(),
            transactions: self.transactions.to_vec(),
            uncles: self.uncles.to_vec(),
        }
    }
}