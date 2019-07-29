use crate::node::package::Package;
use clap::{App, Arg, SubCommand};
use std::error::Error;

pub struct Install {}
impl Install {
    pub fn run(&self, arg_m: Option<&clap::ArgMatches>) -> Result<(), Box<Error>> {
        let package_str = arg_m.unwrap().value_of("package").unwrap();
        let package_name = String::from(package_str);
        let pkg = Package::new(package_name);
        let node_range = pkg.get_node_range()?;
        println!("Node Range for {} - {}", node_range, package_str);
        Ok(())
    }
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("install")
                .visible_alias("i")
                .about("Install a CLI for local use")
                .arg(
                    Arg::with_name("package")
                    .help("NPM package name, github repo URL with binary assets on the release, binary url base with triplet suffix, or direct path to stand-alone binary")
                    .index(1)
                    .required(true)
                )
    }
}
