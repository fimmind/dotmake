use serde::de::DeserializeOwned;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error(transparent)]
    FailedToOpen(#[from] io::Error),

    #[error(transparent)]
    FailedToParse(#[from] serde_yaml::Error),
}

pub fn parse_config<T>(path: &PathBuf) -> Result<T, ParsingError>
where
    T: DeserializeOwned,
{
    Ok(serde_yaml::from_reader(File::open(path)?)?)
}
