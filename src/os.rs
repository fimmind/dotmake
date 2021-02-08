use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::os::unix;
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
        "Process {}",
        .code.map(|code| format!("exited with status code {}", code))
             .unwrap_or("terminated by a signal".to_string())
    )]
    BadExitStatus { code: Option<i32> },

    #[error("Failed to obtain file name of `{0}`")]
    NoFileName(PathBuf),
}

pub fn get_file_name(file: &impl AsRef<Path>) -> Result<&OsStr, OSError> {
    let file = file.as_ref();
    file.file_name().ok_or(OSError::NoFileName(file.to_owned()))
}

pub fn ensure_dir_exists(path: impl AsRef<Path>) -> Result<(), OSError> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path).map_err(|err| OSError::IO {
            msg: format!("Failed to create `{}`", path.display()),
            err,
        })?;
    }
    Ok(())
}

pub fn is_symlink(file: impl AsRef<Path>) -> Result<bool, OSError> {
    let file = file.as_ref();
    let metadata = fs::symlink_metadata(file).map_err(|err| OSError::IO {
        msg: format!("Failed to optain metadata for `{}`", file.display()),
        err,
    })?;
    Ok(metadata.file_type().is_symlink())
}

pub fn move_file(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), OSError> {
    let source = source.as_ref();
    let dest = dest.as_ref();
    fs::rename(source, dest).map_err(|err| OSError::IO {
        msg: format!(
            "Failed to move `{}` to `{}`",
            source.display(),
            dest.display(),
        ),
        err,
    })
}

pub fn remove_file(file: impl AsRef<Path>) -> Result<(), OSError> {
    let file = file.as_ref();
    fs::remove_file(file).map_err(|err| OSError::IO {
        msg: format!("Failed to remove `{}`", file.display()),
        err,
    })
}

pub fn symlink(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), OSError> {
    let source = source.as_ref();
    let dest = dest.as_ref();
    unix::fs::symlink(source, dest).map_err(|err| OSError::IO {
        msg: format!(
            "Failed to crate symlink `{}` -> `{}`",
            source.display(),
            dest.display()
        ),
        err,
    })
}

pub fn open_file(name: impl AsRef<Path>) -> Result<File, OSError> {
    let name = name.as_ref();
    File::open(name).map_err(|err| OSError::IO {
        msg: format!("Failed to open `{}`", name.display()),
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
            msg: format!("Failed to read `{}`", path.display()),
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

pub fn run_shell_script(shell: &str, dir: impl AsRef<Path>, script: &str) -> Result<(), OSError> {
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
        Err(OSError::BadExitStatus {
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
