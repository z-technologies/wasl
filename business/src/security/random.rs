use rand::distributions::Alphanumeric;
use rand::prelude::*;

pub fn generate_random_bytes<'a>(buf: &mut [u8]) {
    thread_rng().fill_bytes(buf);
}

pub fn generate_alphanum_string<const LEN: usize>() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(LEN)
        .map(char::from)
        .collect()
}
