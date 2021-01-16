use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

use crate::cli::OPTIONS;

fn get_default_distro_id() -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(
        process::Command::new("sed")
            .args(&["-n", "s/^ID=//p", "/etc/os-release"])
            .output()?
            .stdout,
    )?
    .trim()
    .to_string())
}

fn get_distro_id() -> Result<String, Box<dyn Error>> {
    match OPTIONS.distro_id() {
        Some(id) => Ok(id.to_string()),
        None => get_default_distro_id(),
    }
}

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(OPTIONS
        .dotfiles_dir()
        .join(format!("dotm-{}.yaml", get_distro_id()?)))
}

pub fn parse_config<T>() -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    Ok(serde_yaml::from_str(&fs::read_to_string(&config_path()?)?)?)
}
