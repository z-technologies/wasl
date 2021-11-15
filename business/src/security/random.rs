use rand::prelude::*;

pub fn generate_random_bytes<'a>(buf: &mut [u8]) {
    thread_rng().fill_bytes(buf);
}
