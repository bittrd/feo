#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;
extern crate reqwest;

mod node;
mod subcommands;

use clap::App;
use std::error::Error;
fn main() -> Result<(), Box<Error>> {
    let app_m = App::new("feo")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(subcommands::install::Install::subcommand())
        .get_matches();
    route_command(app_m)?;
    Ok(())
}

fn route_command(app_m: clap::ArgMatches) -> Result<(),Box<Error>> {
    match app_m.subcommand() {
        ("install", sub_m) => {
            let command = subcommands::install::Install {};
            command.run(sub_m)?;
        }
        _ => {}
    }
    Ok(())
}
