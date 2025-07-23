use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};

pub struct KeyPair {
    pub public_key: PublicKey,
    // this field is kept private so that only enclve functions can access it
    secret_key: SecretKey,
}

impl KeyPair {
    fn new() -> Self {
        let mut r = OsRng;
        let secret_key = SecretKey::random(&mut r);
        let public_key = PublicKey::from_secret_key(&secret_key);
        KeyPair {
            public_key,
            secret_key,
        }
    }

    // this function generates a reference to the secret key and gives to the caller
    // "supposed to be secure"
    fn get_secret_key(&self) -> &SecretKey {
        &self.secret_key
    }
}

pub fn setup() -> (PublicKey, PublicKey, String) {
    // generatinf the key pair for enclave and the local-account
    let manufacturers_key_pair = KeyPair::new();
    let local_key_pair = KeyPair::new();

    // generating the Ethereum address for the treasury
    let serialize = local_key_pair.public_key.serialize();
    let serial_public_key_bytes = &serialize[1..];
    let hash = Keccak256::digest(serial_public_key_bytes);
    let address = hex::encode(&hash[12..]);

    let _msk = manufacturers_key_pair.get_secret_key();
    let _sk = local_key_pair.get_secret_key();

    (
        manufacturers_key_pair.public_key,
        local_key_pair.public_key,
        address,
    )
}
