use clap::{App, Arg, SubCommand};

pub struct Install {}
impl Install {
    pub fn run(&self, arg_m: Option<&clap::ArgMatches>) {
        let package = arg_m.unwrap().value_of("package").unwrap();
        let package_name = String::from(package);
        println!("Installing package: {}", package_name);
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
