use rand::prelude::*;

struct Ascii;

impl Distribution<char> for Ascii {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()[]{}:;"
            .choose(rng)
            .unwrap() as char
    }
}

pub fn generate_random_bytes<'a>(buf: &mut [u8]) {
    thread_rng().fill_bytes(buf);
}

pub fn generate_random_string<D>(len: usize) -> String {
    thread_rng()
        .sample_iter(&Ascii)
        .take(len)
        .map(char::from)
        .collect()
}
