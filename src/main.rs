#![feature(type_alias_impl_trait)]
#![allow(dead_code, unused_variables)] // TODO: Remove it before releasing

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
mod io;
mod cli;
mod config;
mod deps_resolver;
mod identifier;
mod os;

fn main() {
    cli::SUBCOMMAND.perform().unwrap_or_else(exit_error_fn!())
}
