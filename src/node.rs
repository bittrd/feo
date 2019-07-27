extern crate reqwest;

use std::error::Error;

pub struct Manager<'a> {
    base_path: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Versions {
    version: String,
    date: String,
    npm: String,
    lts: String,
    security: String
}

impl<'a> Manager<'a> {
    pub fn new(base_path: &'a String) -> Manager {
        Manager {
            base_path: &base_path,
        }
    }
    pub fn get_latest(&self) -> Result<String, Box<Error>> {
        let versions: Vec<Versions> = reqwest::get("https://nodejs.org/dist/index.json")?.json()?;
        match versions.first() {
            None => bail!("No versions found"),
            Some(ref version) => Ok(version.version.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let expected_path = String::from("my-test-path");
        let manager = Manager::new(&expected_path);
        assert_eq!(expected_path, manager.base_path);
    }

    #[test]
    fn test_get_latest() -> Result<(), Box<Error>> {
        let expected_path = String::from("my-test-path");
        let manager = Manager::new(&expected_path);
        let latest = manager.get_latest()?;
        println!("{}",latest);
        Ok(())
    }
}
