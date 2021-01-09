mod subcommands;

use std::error::Error;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use subcommands::Subcommand;

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

    /// Specify distribution id to use
    #[structopt(short = "D", long = "distro", value_name = "ID", global = true)]
    distro_id: Option<String>,

    /// Use default values for confirmation dialogues
    #[structopt(short = "y", long, global = true)]
    noconfirm: bool,
}

impl Options {
    pub fn dotfiles_dir(&self) -> &PathBuf {
        &self.dotfiles_dir
    }

    fn default_distro_id() -> Result<String, Box<dyn Error>> {
        Ok(String::from_utf8(
            process::Command::new("sed")
                .args(&["-n", "s/^ID=//p", "/etc/os-release"])
                .output()?
                .stdout,
        )?
        .trim()
        .to_string())
    }

    pub fn distro_id(&self) -> Result<String, Box<dyn Error>> {
        Ok(match &self.distro_id {
            Some(id) => id.clone(),
            None => Self::default_distro_id()?,
        })
    }

    pub fn noconfirm(&self) -> bool {
        self.noconfirm
    }
}

/// Dotfiles installation manager
#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(flatten)]
    options: Options,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

impl Cli {
    pub fn init() -> Self {
        Self::from_args()
    }

    pub fn run_subcommand(&self) -> Result<(), Box<dyn Error>> {
        self.subcommand.perform(&self.options)
    }
}
