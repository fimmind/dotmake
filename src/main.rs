//! Dotfiles intallation manager
//!
//! For more information see `README.md` in the [github repository]
//!
//! [github repository]: https://github.com/fimmind/dotmake

#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
mod io;
mod cli;
mod config;
mod identifier;
mod os;

fn main() {
    if let Err(err) = cli::SUBCOMMAND.perform() {
        exit_error!("{}", err);
    }
}
