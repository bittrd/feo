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
        .subcommand(
            SubCommand::with_name("install")
                .visible_alias("i")
                .about("Install a CLI for local use")
                .arg(Arg::with_name("package").help("NPM package name, github URL, binary url base with triplet suffix, or direct path to stand-alone binary").index(1).required(true)),
        )
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
