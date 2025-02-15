use crate::crypto::signature::ECDSASignature;

pub struct Wallet {
    pub key_pair: ECDSASignature,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let key_pair = ECDSASignature::new();
        // Derive an address from the public key (simplified derivation)
        let public_key_bytes = key_pair.verifying_key.to_encoded_point(false);
        let address = format!("0x{}", hex::encode(&public_key_bytes.as_bytes()[1..]));
        Wallet { key_pair, address }
    }

    pub fn sign_message(&self, message: &str) -> String {
        let signature = self.key_pair.sign(message);
        hex::encode(signature.as_ref())
    }
}
