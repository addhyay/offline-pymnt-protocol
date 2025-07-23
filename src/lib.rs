use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;

pub struct KeyPair {
    pub public_key: PublicKey,
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
