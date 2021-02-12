//! CLI abstraction

mod options;
mod subcommand;

use lazy_static::lazy_static;
use structopt::StructOpt;

use options::Options;
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
