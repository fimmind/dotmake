//! Subcommand that moves a given file to Dotifiles direcotry and crates a
//! symlink instead

use crate::types::UserPath;
use crate::cli;
use crate::os::{get_file_name, move_file, symlink};
use std::error::Error;
use std::ffi::OsString;
use structopt::StructOpt;

/// Move a file to dotfiles directory, replacing it with a symlink
#[derive(Debug, StructOpt)]
pub struct Add {
    /// A file to move to your dotfiles
    #[structopt(required = true)]
    file: UserPath,

    /// Store the file with a given name in the dotfiles directory
    #[structopt(long = "with_name", short = "o")]
    with_name: Option<OsString>,
}

impl Add {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        let dest = cli::options()
            .dotfiles_dir()
            .join(match &self.with_name {
                Some(name) => name,
                None => get_file_name(&self.file)?,
            });

        if dest.exists() {
            print_warn!("File `{}` already exists", dest.display());
            if !confirm!("Replace it?"; true) {
                print_info!("Aborting...");
                return Ok(());
            }
            print_info!("Replacing `{}` with a newly added file", dest.display());
        } else {
            print_info!("Moving `{}` to your dotfiles", dest.display());
        }
        move_file(&self.file, &dest)?;

        let dest = dest.canonicalize()?;
        print_info!(
            "Creating symlink `{}` -> `{}`",
            self.file.display(),
            dest.display()
        );
        symlink(dest, &self.file)?;
        Ok(())
    }
}
