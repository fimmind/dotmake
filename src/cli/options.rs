use std::path::PathBuf;
use structopt::StructOpt;

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

    pub fn distro_id(&self) -> Option<&str> {
        self.distro_id.map(|s| s.as_str())
    }

    pub fn noconfirm(&self) -> bool {
        self.noconfirm
    }
}
