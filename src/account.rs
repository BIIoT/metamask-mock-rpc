use crypto::ecdsa::private_key::PrivateKey;
use crypto::ecdsa::public_key::PublicKey;
use ethereum_types::{Address, U256};

pub struct Account {
    pk: PrivateKey,
    pubk: PublicKey,
}

impl Account {
    /// `sign_transaction` signs the transaction using the given signer and private key.
    pub fn sign_transaction(&self, nonce: u64, gas_price: U256, gas: u64, to: Address, value: U256, data: Vec<u8>, chain_id: U256) {

    }
}