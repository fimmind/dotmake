//! An action that creates soft links to actions dotfiles

use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use crate::config::deserializers::List;
use crate::os::{self, OSError};
use std::collections::HashMap;
use std::env::set_current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Links {
    links: HashMap<PathBuf, List<PathBuf>>,
}

impl Action for Links {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        set_current_dir(OPTIONS.dotfiles_dir())?;
        for (source, dests) in &self.links {
            for dest in dests.iter() {
                if dest.exists() {
                    if os::is_symlink(dest)? {
                        os::remove_file(dest)?;
                    } else {
                        backup(dest, &conf.backup_dir)?;
                    }
                }
                os::symlink(source, dest)?;
            }
        }
        Ok(())
    }
}

fn backup(file: &Path, backup_dir: &Path) -> Result<(), OSError> {
    os::ensure_dir_exists(backup_dir)?;
    let backup_file_path = get_backup_file_path(file, backup_dir)?;
    os::move_file(file, &backup_file_path)
}

fn get_backup_file_path(file: &Path, backup_dir: &Path) -> Result<PathBuf, OSError> {
    let fname = os::get_file_name(&file)?;
    let mut backup_file_path = backup_dir.join(fname);

    let mut i = 1u128;
    while backup_file_path.exists() {
        let mut next_fname = fname.to_os_string();
        next_fname.push(&format!(" ({})", i));
        backup_file_path.set_file_name(next_fname);
        i += 1;
    }

    Ok(backup_file_path)
}
