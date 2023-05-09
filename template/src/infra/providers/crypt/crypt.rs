use bcrypt::{hash, verify};

use super::crypt_trait::CryptTrait;

pub struct CrypterAPP;

impl CryptTrait for CrypterAPP {
    fn encrypt(to_encrypt: String) -> String {
        hash(to_encrypt, 4).unwrap()
    }

    fn check_hash(no_hash: String, hash: String) -> bool {
        verify(no_hash, &hash).unwrap()
    }
}