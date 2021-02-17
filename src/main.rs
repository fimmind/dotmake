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
mod types;
mod os;

fn main() {
    if let Err(err) = cli::subcommand().perform() {
        exit_error!("{}", err);
    }
}
