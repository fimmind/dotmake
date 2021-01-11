use config::{Config, File};
use serde::Deserialize;
use std::error::Error;
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
        None => get_default_distro_id()
    }
}

fn get_raw_config() -> Result<Config, Box<dyn Error>> {
    let mut raw_config = Config::new();
    raw_config.merge(File::from(
        OPTIONS
            .dotfiles_dir()
            .join(format!("dotm-{}.yaml", get_distro_id()?)),
    ))?;
    Ok(raw_config)
}

pub fn parse_config<'de, T>() -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'de>,
{
    Ok(get_raw_config()?.try_into()?)
}
