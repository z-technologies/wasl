use crate::result::Result;

pub fn get_pem_contents(pem_key: &[u8]) -> Result<Vec<u8>> {
    Ok(pem::parse(pem_key)?.contents)
}
