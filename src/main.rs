#![feature(type_alias_impl_trait)]
#![allow(dead_code, unused_variables)] // TODO: Remove it before releasing

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
