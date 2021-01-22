use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OSError {
    #[error("{msg}: {err}")]
    IO {
        #[source]
        err: io::Error,
        msg: String,
    },

    #[error(
        "Process exited with {}",
        .code.map(|code| format!("shell code {}", code))
             .unwrap_or("error".to_string())
    )]
    ExitError { code: Option<i32> },
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
            return Ok(line[3..].trim().to_string());
        }
    }
    Ok("linux".to_string())
}

pub fn run_shell_script(shell: &str, dir: &PathBuf, script: &str) -> Result<(), OSError> {
    let shell_err = |err| OSError::IO {
        msg: "Shell error".to_string(),
        err,
    };
    let mut shell = Command::new(shell)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(shell_err)?;

    write!(shell.stdin.as_mut().unwrap(), "{}", script).map_err(shell_err)?;

    let exit_status = shell.wait().map_err(shell_err)?;
    if !exit_status.success() {
        Err(OSError::ExitError {
            code: exit_status.code(),
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_distro_id;
    use super::OSError;

    #[test]
    fn distro_id() -> Result<(), OSError> {
        println!("Current distro ID: {}", get_distro_id()?);
        Ok(())
    }
}
