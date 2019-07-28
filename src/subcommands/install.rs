use clap::{App, Arg, SubCommand};

pub struct Command {}
impl Command {
    pub fn run(&self, package: String) {
        println!("Installing package: {}", package);
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
