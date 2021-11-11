use argon2::{self, Config};
use hex;

const PASSWORD_HASH_LENGTH: u32 = 32;

pub fn hash_password<'a>(password: &'a str, salt: &'a str) -> String {
    let mut config = Config::default();
    config.hash_length = PASSWORD_HASH_LENGTH;

    let password_bytes = hex::decode(password).unwrap();
    let salt_bytes = hex::decode(salt).unwrap();
    let encoded =
        argon2::hash_encoded(&password_bytes, &salt_bytes, &config).unwrap();

    hex::encode(encoded)
}

pub fn password_matches<'a>(
    password: &'a str,
    password_hash: &'a str,
    password_salt: &'a str,
) -> bool {
    hash_password(password, password_salt) == password_hash
}
