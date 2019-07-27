#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod node;

use node::Manager;
use std::error::Error;
fn main() -> Result<(), Box<Error>> {
    let manager_path = String::from("~/.feo");
    let manager = Manager::new(&manager_path);
    let latest_version = manager.get_latest()?;
    println!("Latest node version: {}", latest_version);
    Ok(())
}
