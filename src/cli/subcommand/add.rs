//! Subcommand that moves a given file to Dotifiles direcotry and crates a
//! symlink instead

use crate::cli;
use crate::os::{get_file_name, move_file, symlink};
use crate::types::UserPath;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
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
        // Remove trailing slashes to prevent potential errors when linking
        let file = self.file.components().as_path();
        let with_name = match &self.with_name {
            Some(name) => Path::new(name).components().as_path().as_os_str(),
            None => get_file_name(&file)?,
        };

        let dest = cli::options().dotfiles_dir().join(&with_name);

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
        move_file(&file, &dest)?;

        let dest = dest.canonicalize()?;
        print_info!(
            "Creating symlink `{}` -> `{}`",
            file.display(),
            dest.display()
        );
        symlink(dest, &file)?;
        Ok(())
    }
}
