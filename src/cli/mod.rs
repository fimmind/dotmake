//! CLI abstraction

mod options;
mod subcommand;

use once_cell::sync::Lazy;
use structopt::StructOpt;

use options::Options;
use subcommand::Subcommand;

/// Lazily parsed CLI
static CLI: Lazy<Cli> = Lazy::new(Cli::from_args);

/// Get lazily parsed CLI options
pub fn options() -> &'static Options {
    &CLI.options
}

/// Get lazily parsed CLI subcommand
pub fn subcommand() -> &'static Subcommand {
    &CLI.subcommand
}

/// Simple wrapper joining [`Options`] and [`Subcommand`]
#[derive(Debug, StructOpt)]
#[structopt(about = "Dotfiles installation manager")]
pub struct Cli {
    #[structopt(flatten)]
    options: Options,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}
