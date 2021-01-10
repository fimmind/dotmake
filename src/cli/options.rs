use structopt::StructOpt;
use std::error::Error;
use std::path::PathBuf;
use std::process;

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

    fn get_default_distro_id() -> Result<String, Box<dyn Error>> {
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
        match &self.distro_id {
            Some(id) => Ok(id.clone()),
            None => Self::get_default_distro_id(),
        }
    }

    pub fn noconfirm(&self) -> bool {
        self.noconfirm
    }
}
