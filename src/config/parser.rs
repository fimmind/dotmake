use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Failed to read configuration file: {0}")]
    FailedToRead(#[from] io::Error),

    #[error("Failed to parse configuration file: {0}")]
    FailedToParse(#[from] serde_yaml::Error),
}

pub fn parse_config<T>(path: &PathBuf) -> Result<T, ParsingError>
where
    T: DeserializeOwned,
{
    Ok(serde_yaml::from_str(&fs::read_to_string(path)?)?)
}
