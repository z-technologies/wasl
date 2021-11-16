use crate::result::Result;

use std::fs::File;
use std::io::Read;

pub fn load_file_bytes(path: &str) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;

    file.read_to_end(&mut buf)?;
    Ok(buf)
}
