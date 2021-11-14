use crate::security::random::generate_random_bytes;

use argon2::{self, Config};
use base64;

pub fn is_match<'a>(password: &'a str, password_hash: &'a str) -> bool {
    let password_hash = base64::decode(password_hash).unwrap();
    let password_encoded = std::str::from_utf8(&password_hash).unwrap();

    argon2::verify_encoded(password_encoded, &password.as_bytes())
        .unwrap_or(false)
}

pub fn make_password_hash<'a>(password: &'a str) -> String {
    let mut salt_buf: [u8; 16] = [0u8; 16];
    generate_random_bytes(&mut salt_buf);

    hash_password(password, &salt_buf)
}

fn hash_password<'a>(password: &'a str, salt: &'a [u8]) -> String {
    let config = Config::default();

    let bytes = password.as_bytes();
    let encoded = argon2::hash_encoded(&bytes, &salt, &config).unwrap();

    base64::encode(encoded)
}
