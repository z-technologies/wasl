use crate::result::Result;

pub fn get_environment_value(key: &str) -> Result<String> {
    Ok(std::env::var(key)?)
}
