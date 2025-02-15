use k256::ecdsa::{SigningKey, VerifyingKey, Signature, signature::{Signer, Verifier}};
use rand::rngs::OsRng;

pub struct ECDSASignature {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl ECDSASignature {
    /// Generates a new ECDSA key pair.
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        ECDSASignature { signing_key, verifying_key }
    }

    /// Signs a message and returns the signature.
    pub fn sign(&self, message: &str) -> Signature {
        self.signing_key.sign(message.as_bytes())
    }

    /// Verifies the given signature against the message.
    pub fn verify(&self, message: &str, signature: &Signature) -> bool {
        self.verifying_key.verify(message.as_bytes(), signature).is_ok()
    }
}
