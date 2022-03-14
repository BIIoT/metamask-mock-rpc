use std::str::FromStr;
use crypto::hash::keccak256;
use crypto::ecdsa::public_key::PublicKey;
use ethereum_types::{Address, H256, U256, H512};
use log::{debug, info};
use rlp::{Encodable, Decodable, RlpStream, Rlp, DecoderError};
use serde::{Serialize, Deserialize};
use crate::methods::eth_chain_id::CHAIN_ID;

pub struct MetamaskRawTransaction {
    pub nonce: Vec<u8>,
    pub gas_price: Vec<u8>,
    pub gas: Vec<u8>,
    pub recipient: Vec<u8>, // in Ethereum, it called 'to'
    pub value: Vec<u8>,
    pub data: Vec<u8>, // 6080 6040 ..
    pub v: Vec<u8>,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl Encodable for MetamaskRawTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.recipient);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl Decodable for MetamaskRawTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Self {
            nonce: rlp.val_at(0).unwrap(),
            gas_price: rlp.val_at(1).unwrap(),
            gas: rlp.val_at(2).unwrap(),
            recipient: rlp.val_at(3).unwrap(),
            value: rlp.val_at(4).unwrap(),
            data: rlp.val_at(5).unwrap(),
            v: rlp.val_at(6).unwrap(),
            r: rlp.val_at(7).unwrap(),
            s: rlp.val_at(8).unwrap(),
        })
    }
}

pub struct RawTransactionData {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas: U256,
    pub recipient: Vec<u8>,
    pub value: U256,
    pub data: Vec<u8>,
    pub chain_id: u64,
}

impl From<&RawTransaction> for RawTransactionData {
    fn from(rtx: &RawTransaction) -> Self {
        Self {
            nonce: rtx.nonce.clone(),
            gas_price: rtx.gas_price.clone(),
            gas: rtx.gas.clone(),
            recipient: rtx.recipient.to_fixed_bytes().to_vec(),
            value: rtx.value.clone(),
            data: rtx.data.to_vec(),
            chain_id: u64::from_str_radix(&CHAIN_ID.clone()[2..], 16).unwrap(),
        }
    }
}

impl Encodable for RawTransactionData {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.recipient);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.chain_id); // chain id
        s.append(&0u64);
        s.append(&0u64);
        // s.append(&0u64); // empty r
        // s.append(&0u64); // empty s
        // s.append(&true);
        // s.append(&self.chain_id);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawTransaction {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas: U256,
    pub recipient: Address, // in Ethereum, it called 'to'
    pub value: U256,
    pub data: Vec<u8>, // 6080 6040 ..
    pub v: u32,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl RawTransaction {
    pub fn sender(&self) -> Address {
        let rlp_tx = rlp::encode(self).to_vec();
        let msg_digest = keccak256(rlp_tx.as_slice());
        let r = H256::from_slice(self.r.as_slice());
        let s = H256::from_slice(self.s.as_slice());
        let signature = make_signature(r, s);
        let pubkey = PublicKey::from_signature(
            self.v as i32,
            signature.as_fixed_bytes(),
            msg_digest.as_fixed_bytes()).unwrap();
        return pubkey.address();
    }
}

impl From<MetamaskRawTransaction> for RawTransaction {
    fn from(mrtx: MetamaskRawTransaction) -> Self {
        let mut _str_nonce = "0".to_string();
        if mrtx.nonce.len() != 0 {
            // 2가 입력으로 올 때, 이것을 ASCII 2가 아닌 Integer 2로 인식해야 한다.
            // 1 2F (=0x12F)가 입력으로 올 때, 이것을 BE 형태의 Integer 2로 인식해야 한다.
            _str_nonce = hex::encode(mrtx.nonce.to_vec());
        }
        print!("{}", _str_nonce.as_str());
        let nonce = u64::from_str_radix(_str_nonce.as_str(), 16).unwrap();
        let mut gas_price = U256::zero();
        if mrtx.gas_price.len() != 0 {
            gas_price = U256::from_big_endian(mrtx.gas_price.as_ref());
        }
        let mut gas = U256::zero();
        if mrtx.gas.len() != 0 {
            gas = U256::from_big_endian(mrtx.gas.as_ref());
        }
        let mut recipient = Address::zero();
        if mrtx.recipient.len() != 0 {
            recipient = Address::from_slice(mrtx.recipient.as_ref());
        }
        let mut value = U256::zero();
        if mrtx.value.len() != 0 {
            value = U256::from_big_endian(mrtx.value.as_ref());
        }
        let mut _str_v = "0".to_string();
        if mrtx.v.len() != 0 {
            _str_v = hex::encode(mrtx.v.to_vec());
        }
        info!("v: {}", _str_v);
        let v = u32::from_str_radix(_str_v.as_str(), 16).unwrap();

        Self {
            nonce, gas_price, gas, recipient, value,
            data: mrtx.data.to_vec(),
            v,
            r: mrtx.r.to_vec(),
            s: mrtx.s.to_vec(),
        }
    }
}

impl Encodable for RawTransaction {
    /// Encoded RawTransaction will be sent from Web3 client.
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.recipient);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl Decodable for RawTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(RawTransaction {
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas: rlp.val_at(2)?,
            recipient: rlp.val_at(3)?,
            value: rlp.val_at(4)?,
            data: rlp.val_at(5)?,
            v: rlp.val_at(6)?,
            r: rlp.val_at(7)?,
            s: rlp.val_at(8)?
        })
    }
}

impl Clone for RawTransaction {
    fn clone(&self) -> Self {
        Self {
            nonce: self.nonce.clone(),
            gas_price: self.gas_price.clone(),
            gas: self.gas.clone(),
            recipient: self.recipient.clone(),
            value: self.value.clone(),
            data: self.data.clone(),
            v: self.v.clone(),
            r: self.r.to_vec(),
            s: self.s.to_vec(),
        }
    }
}

fn make_signature(r: H256, s: H256) -> H512 {
    let mut v512 = vec![];
    for c in r.as_fixed_bytes() { v512.push(c.clone()); }
    for c in s.as_fixed_bytes() { v512.push(c.clone()); }
    return H512::from_slice(v512.as_slice());
}