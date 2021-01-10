mod subcommand;
mod options;

use structopt::StructOpt;
use lazy_static::lazy_static;

use subcommand::Subcommand;
use options::Options;

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
