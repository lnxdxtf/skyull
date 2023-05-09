pub trait CryptTrait {
    fn encrypt(to_encrypt: String) -> String;
    fn check_hash(no_hash: String, hash: String) -> bool;
}
