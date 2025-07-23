use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};

// this struct will create asymmetric key pair over standard secp256k1 curve
pub struct KeyPair {
    pub public_key: PublicKey,
    _secret_key: SecretKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let mut r = OsRng;
        let _secret_key = SecretKey::random(&mut r);
        let public_key = PublicKey::from_secret_key(&_secret_key);
        KeyPair {
            public_key,
            _secret_key,
        }
    }

    // gets the private key
    fn _get_secret_key(&self) -> &SecretKey {
        &self._secret_key
    }
}

// generates the Ethereum address from public key of treasury
pub fn generate_address(public_key: PublicKey) -> String {
    let hash = Keccak256::digest(&public_key.serialize()[1..]);
    String::from("0x") + hex::encode(&hash[12..]).as_str()
}
