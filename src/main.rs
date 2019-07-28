#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

mod node;
mod subcommands;

use clap::{Arg, SubCommand};
use std::error::Error;
fn main() -> Result<(), Box<Error>> {
    let app_m = clap::App::new("feo")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(subcommands::install::Command::subcommand())
        .subcommand(SubCommand::with_name("node").about("Manage node.js used for feo").arg(Arg::with_name("version")))
        .get_matches();
    match app_m.subcommand() {
        ("install", sub_m) => {
            let command = subcommands::install::Command {};
            let package = sub_m.unwrap().value_of("package").unwrap();
            command.run(String::from(package));
        }
        _ => {}
    }
    Ok(())
}
