use crate::result::Result;

use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey,
};

pub struct ECDSASignature {
    key: SigningKey,
}

impl ECDSASignature {
    pub fn new(key: &[u8]) -> Result<Self> {
        Ok(Self {
            key: SigningKey::from_bytes(key)?,
        })
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.key.sign(msg)
    }

    pub fn verify(&self, msg: &[u8], signature: &Signature) -> Result<()> {
        Ok(self.key.verifying_key().verify(msg, signature)?)
    }
}
