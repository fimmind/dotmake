use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OSError {
    #[error("{msg}: {err}")]
    IO {
        #[source]
        err: io::Error,
        msg: String,
    },
}

pub fn open_file(name: impl AsRef<Path>) -> Result<File, OSError> {
    File::open(name.as_ref()).map_err(|err| OSError::IO {
        msg: format!("Failed to open '{:?}'", name.as_ref()),
        err,
    })
}

pub fn read_file(
    name: impl AsRef<Path>,
) -> Result<impl Iterator<Item = Result<String, OSError>>, OSError> {
    let path: PathBuf = name.as_ref().into();
    let reader = BufReader::new(open_file(&name)?);
    Ok(reader.lines().map(move |line| {
        line.map_err(|err| OSError::IO {
            msg: format!("Failed to read '{:?}'", path),
            err,
        })
    }))
}

pub fn get_distro_id() -> Result<String, OSError> {
    for line in read_file("/etc/os-release")? {
        let line = line?;
        if line.starts_with("ID=") {
            return Ok(line[3..].to_string());
        }
    }
    Ok("linux".to_string())
}

#[cfg(test)]
mod tests {
    use super::OSError;
    use super::get_distro_id;

    #[test]
    fn distro_id() -> Result<(), OSError> {
        println!("Current distro ID: {}", get_distro_id()?);
        Ok(())
    }
}
