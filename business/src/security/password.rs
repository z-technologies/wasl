use hex;
use sha2::{Digest, Sha256};

pub fn hash_password<'a>(password: &'a str, salt: &'a str) -> String {
    let mut hasher = Sha256::default();

    hasher.update(password);
    hasher.update(salt);

    hex::encode(hasher.finalize())
}

pub fn password_matches<'a>(
    password: &'a str,
    password_hash: &'a str,
    password_salt: &'a str,
) -> bool {
    hash_password(password, password_salt) == password_hash
}
