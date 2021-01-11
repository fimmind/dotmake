#![feature(type_alias_impl_trait)]

#[macro_use]
mod io;
mod cli;
mod config;

fn main() {
    cli::SUBCOMMAND.perform().unwrap_or_else(exit_error_fn!())
}
