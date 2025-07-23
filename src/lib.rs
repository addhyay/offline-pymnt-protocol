use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;

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

pub fn setup() -> (PublicKey, PublicKey) {
    let manufacturers_key_pair = KeyPair::new();
    let local_key_pair = KeyPair::new();

    let _msk = manufacturers_key_pair.get_secret_key();
    let _sk = local_key_pair.get_secret_key();

    (manufacturers_key_pair.public_key, local_key_pair.public_key)
}
