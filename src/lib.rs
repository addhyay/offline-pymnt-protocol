// modules
mod cryptography;

// imports
use crate::cryptography::{generate_address, KeyPair};
use libsecp256k1::PublicKey;

pub fn setup() -> (PublicKey, PublicKey, String) {
    // generating the key pair for enclave and the local-account
    let manufacturers_key_pair = KeyPair::new();
    let local_key_pair = KeyPair::new();

    (
        manufacturers_key_pair.public_key,
        local_key_pair.public_key,
        generate_address(local_key_pair.public_key),
    )
}
