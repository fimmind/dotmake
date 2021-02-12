//! CLI abstraction

mod subcommand;

use crate::os::{self, OSError};
use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;

use subcommand::Subcommand;

lazy_static! {
    /// Lazily parsed CLI arguments
    static ref CLI: Cli = Cli::from_args();

    /// Lazily parsed CLI options
    pub static ref OPTIONS: &'static Options = &CLI.options;

    /// Lazily parsed CLI subcommand
    pub static ref SUBCOMMAND: &'static Subcommand = &CLI.subcommand;
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Dotfiles installation manager")]
pub struct Cli {
    #[structopt(flatten)]
    options: Options,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Set a custom dotfiles directory
    #[structopt(
        short = "d",
        long = "dotdir",
        env = "DOTM_DOTFILES_DIR",
        value_name = "DIR",
        default_value = "./",
        global = true
    )]
    dotfiles_dir: PathBuf,

    /// Specify linux distribution id to use
    #[structopt(short = "D", long = "distro", value_name = "ID", global = true)]
    distro_id: Option<String>,

    /// Use default values for confirmation dialogues
    #[structopt(short = "y", long, global = true)]
    noconfirm: bool,
}

impl Options {
    /// Getter for dotfiles directory
    pub fn dotfiles_dir(&self) -> &PathBuf {
        &self.dotfiles_dir
    }

    /// Getter for `noconfirm` option
    pub fn noconfirm(&self) -> bool {
        self.noconfirm
    }

    /// Getter for linux distro identifier
    ///
    /// If linux distro isn't specified by the user, it's determined using
    /// [`crate::os::get_distro_id`]
    pub fn distro_id(&self) -> Result<String, OSError> {
        match &self.distro_id {
            Some(id) => Ok(id.to_string()),
            None => os::get_distro_id(),
        }
    }
}
