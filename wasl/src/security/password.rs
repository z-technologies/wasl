use crate::result::Result;
use crate::security::random::generate_random_bytes;

use argon2;
use base64;

pub fn is_match<'a>(password: &'a str, hash: &'a str) -> Result<bool> {
    let hash = base64::decode(hash)?;
    let encoded = std::str::from_utf8(&hash)?;

    Ok(argon2::verify_encoded(encoded, &password.as_bytes())?)
}

pub fn make_hash<'a>(password: &'a str) -> Result<String> {
    let mut salt_buf: [u8; 16] = [0u8; 16];
    generate_random_bytes(&mut salt_buf);

    hash_password(password, &salt_buf)
}

fn hash_password<'a>(password: &'a str, salt: &'a [u8]) -> Result<String> {
    let config = argon2::Config::default();

    let bytes = password.as_bytes();
    let encoded = argon2::hash_encoded(&bytes, &salt, &config)?;

    Ok(base64::encode(encoded))
}
