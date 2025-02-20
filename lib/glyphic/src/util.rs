use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("TOML Parsing Error")]
    TomlError(#[from] toml::de::Error),
}

pub fn load_config_from_file<T>(path: &str) -> Result<T, Error>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let file_contents = fs::read_to_string(&path).expect("Could not read input file");
    let config: T = toml::from_str(&file_contents)?;
    Ok(config)
}
