//! Various functions providing more convenient way of interacting with OS and
//! it's file system

use once_cell::sync::OnceCell;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use thiserror::Error;

/// Various errors that can occure while interating with OS
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

    #[error("File `{0}` exists but is not a directory")]
    NotADirectory(PathBuf),

    #[error("Failed to obtain file name of `{0}`")]
    NoFileName(PathBuf),
}

/// Get file name from the given path
///
/// # Example
/// ```
/// assert_eq!(get_file_name("./foo/bar/test.txt"), OsStr::from("test.txt"))
/// ```
pub fn get_file_name(file: &impl AsRef<Path>) -> Result<&OsStr, OSError> {
    let file = file.as_ref();
    file.file_name()
        .ok_or_else(|| OSError::NoFileName(file.to_owned()))
}

/// Ensure that the given file exists and is a directory
///
/// - If `path` does not exist, it's created using [`std::fs::create_dir_all`].
/// - If it exists but is not a directory, an error is returned.
pub fn ensure_dir_exists(path: impl AsRef<Path>) -> Result<(), OSError> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path).map_err(|err| OSError::IO {
            msg: format!("Failed to create `{}`", path.display()),
            err,
        })?;
    } else if !path.is_dir() {
        return Err(OSError::NotADirectory(path.to_owned()));
    }
    Ok(())
}

/// Test, whether or not the given file is a symlink
///
/// If path does not exist, `false` is returned
///
/// # Example
/// ```
/// assert_eq!(is_symlink("not_a_symlink.txt", Ok(false)));
/// assert_eq!(is_symlink("/does_not_exist.asdf"), Ok(false));
/// assert_eq!(is_symlink("symlink.yaml", Ok(true)));
/// ```
pub fn is_symlink(file: impl AsRef<Path>) -> Result<bool, OSError> {
    let file = file.as_ref();
    if !file.exists() {
        return Ok(false);
    }
    let metadata = file.symlink_metadata().map_err(|err| OSError::IO {
        msg: format!("Failed to obtain metadata for `{}`", file.display()),
        err,
    })?;
    Ok(metadata.file_type().is_symlink())
}

/// A wrapper aroung [`std::fs::rename`] providing more informative error
/// messages
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

/// A wrapper aroung [`std::fs::remove_file`] providing more informative error
/// messages
pub fn remove_file(file: impl AsRef<Path>) -> Result<(), OSError> {
    let file = file.as_ref();
    fs::remove_file(file).map_err(|err| OSError::IO {
        msg: format!("Failed to remove `{}`", file.display()),
        err,
    })
}

/// A wrapper aroung [`std::os::unix::fs::symlink`] providing more informative
/// error messages
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

/// Open file
pub fn open_file(name: impl AsRef<Path>) -> Result<File, OSError> {
    let name = name.as_ref();
    File::open(name).map_err(|err| OSError::IO {
        msg: format!("Failed to open `{}`", name.display()),
        err,
    })
}

/// Read file line-by-line
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

/// Cache for [`get_distro_id`]
static DISTRO_ID: OnceCell<String> = OnceCell::new();

/// Read `ID` field of `/etc/os-release`
///
/// If the field is not set, "linux" is returned
pub fn get_distro_id() -> Result<&'static str, OSError> {
    DISTRO_ID
        .get_or_try_init(|| {
            for line in read_file("/etc/os-release")? {
                let line = line?;
                if line.starts_with("ID=") {
                    return Ok(line[3..].trim().to_string());
                }
            }
            Ok("linux".to_string())
        })
        .map(|i| i.as_str())
}

/// Run shell scrip in the given directory
///
/// # Errors
/// Incomprehensive list of possible error cases:
/// - `shell` is not a valid executable;
/// - `dir` does not exist or is not a directory;
/// - piping to shell process fails;
/// - script fails during execution.
///
/// # Example
/// ```
/// run_shell_script("bash", "./", "echo hello").unwrap(); // prints "hello" to the screen
/// run_shell_script("fish", "/etc", "echo (pwd)").unwrap(); // prints "/etc" to the screen
/// ```
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
