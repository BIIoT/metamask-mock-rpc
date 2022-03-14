use crypto::ecdsa::public_key::PublicKey;
use ethereum_types::H256;
use secp256k1::{Message, Secp256k1, SecretKey};
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
use crate::error::{RecoveryError, SigningError};

pub struct Signature {
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

pub fn sign(secret_key: &SecretKey, message: &[u8], chain_id: u64) -> Result<Signature, SigningError> {
    let message = Message::from_slice(message).map_err(|_| SigningError::InvalidMessage).unwrap();
    let (rec_id, signature) = Secp256k1::new().sign_recoverable(&message, secret_key).serialize_compact();

    let standard_v = rec_id.to_i32() as u64;
    let v = standard_v + 35 + chain_id * 2;
    let r = H256::from_slice(&signature[..32]);
    let s = H256::from_slice(&signature[32..]);

    Ok(Signature { v, r, s })
}

pub fn recover(message: &[u8], signature: &[u8], recovery_id: i32) -> Result<PublicKey, RecoveryError> {
    let message_result = Message::from_slice(message).map_err(|_| RecoveryError::InvalidMessage);
    match message_result {
        Ok(message) => {
            let rec_id_result = RecoveryId::from_i32(recovery_id).map_err(|_| RecoveryError::InvalidSignature);
            match rec_id_result {
                Ok(rec_id) => {
                    let signature_result = RecoverableSignature::from_compact(&signature, rec_id)
                        .map_err(|_| RecoveryError::InvalidSignature);
                    match signature_result {
                        Ok(signature) => {
                            let public_key_result = Secp256k1::new()
                                .recover(&message, &signature)
                                .map_err(|_| RecoveryError::InvalidSignature);
                            match public_key_result {
                                Ok(public_key) => Ok(PublicKey::from(public_key.serialize_uncompressed())),
                                Err(e) => Err(e),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e)
    }
}
