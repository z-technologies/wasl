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

    pub fn sign_base64(&self, msg: &[u8]) -> String {
        use p256::ecdsa::signature::Signature;

        base64::encode(self.sign(msg).as_bytes())
    }

    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<()> {
        let signature =
            <Signature as p256::ecdsa::signature::Signature>::from_bytes(
                signature,
            )?;

        Ok(self.key.verifying_key().verify(msg, &signature)?)
    }
}
