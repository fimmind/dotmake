mod options;
mod subcommand;

use lazy_static::lazy_static;
use structopt::StructOpt;

use options::Options;
use subcommand::Subcommand;

lazy_static! {
    static ref CLI: Cli = Cli::from_args();
    pub static ref OPTIONS: &'static Options = &CLI.options;
    pub static ref SUBCOMMAND: &'static Subcommand = &CLI.subcommand;
}

/// Dotfiles installation manager
#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(flatten)]
    options: Options,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}
