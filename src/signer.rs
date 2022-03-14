use crypto::hash::keccak256;
use ethereum_types::{Address, H256, U256};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use crate::raw_transaction::RawTransaction;

pub struct Eip155Signer {
    chain_id: U256
}

impl Eip155Signer {
    pub fn new() -> Self {
        Self { chain_id: U256::from(9u8) }
    }

    pub fn sign(nonce: u64, gas_price: U256, gas: u64, to: Address, value: U256, data: Vec<u8>) -> H256 {
        let eip155_tx = Eip155Tx::new(nonce, gas_price.clone(), gas, to.clone(), value.clone(), data.to_vec(), 84);
        let rlp_eip155_tx = rlp::encode(&eip155_tx);
        let h_tx = keccak256(rlp_eip155_tx.as_ref());
        return h_tx;
    }

    pub fn sign_from_eip155tx(eip155tx: Eip155Tx) -> H256 {
        let rlp_eip155_tx = rlp::encode(&eip155tx);
        let h_tx = keccak256(rlp_eip155_tx.as_ref());
        return h_tx;
    }
}

pub struct Eip155Tx {
    nonce: u64,
    gas_price: U256,
    gas: u64,
    to: Address,
    value: U256,
    data: Vec<u8>,

    chain_id: u64,
    a: usize,
    b: usize,
}

impl Eip155Tx {
    pub fn new(nonce: u64, gas_price: U256, gas: u64, to: Address, value: U256, data: Vec<u8>, chain_id: u64) -> Self {
        Self { nonce, gas_price, gas, to, value, data, chain_id, a: 0, b: 0 }
    }
}

impl From<RawTransaction> for Eip155Tx {
    fn from(rtx: RawTransaction) -> Self {
        Self {
            nonce: rtx.nonce.clone(),
            gas_price: rtx.gas_price.clone(),
            gas: rtx.gas.as_u64(),
            to: rtx.recipient.clone(),
            value: rtx.value.clone(),
            data: rtx.data.to_vec(),

            chain_id: 84,
            a: 0,
            b: 0,
        }
    }
}

impl Encodable for Eip155Tx {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.to);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.a);
        s.append(&self.b);
    }
}

impl Decodable for Eip155Tx {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Self {
            nonce: rlp.val_at(0).unwrap(),
            gas_price: rlp.val_at(1).unwrap(),
            gas: rlp.val_at(2).unwrap(),
            to: rlp.val_at(3).unwrap(),
            value: rlp.val_at(4).unwrap(),
            data: rlp.val_at(5).unwrap(),
            chain_id: 0,
            a: rlp.val_at(6).unwrap(),
            b: rlp.val_at(7).unwrap(),
        })
    }
}