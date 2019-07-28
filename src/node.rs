extern crate reqwest;

use serde_json::Value;
use std::error::Error;

pub struct Manager {
    base_path: String,
    node_base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Versions {
    version: String,
    date: String,
    lts: Value,
    security: bool,
}

impl Manager {
    pub fn new(base_path: String) -> Manager {
        Manager {
            base_path: base_path,
            node_base_url: String::from("https://nodejs.org/dist/index.json"),
        }
    }
    fn get_version_json(&self) -> Result<Vec<Versions>,Box<Error>> {
        let url = format!("{}{}", self.node_base_url, "/dist/index.json");
        let versions: Vec<Versions> = reqwest::get(&url)?.json()?;
        Ok(versions)
    }
    pub fn get_latest(&self) -> Result<String, Box<Error>> {
        let versions = self.get_version_json()?;
        match versions.first() {
            None => bail!("No versions found"),
            Some(ref version) => Ok(version.version.clone()),
        }
    }
    pub fn get_lts(&self) -> Result<String, Box<Error>> {
        let versions = self.get_version_json()?;
        for version in versions {
            if version.lts.is_boolean() && version.lts.as_bool().unwrap_or(false) {
                return Ok(version.version);
            }
            if version.lts.is_string() {
                return Ok(version.version);
            }
        };
        bail!("No LTS versions found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use mockito::mock;

    fn create_manager() -> Manager {
        Manager {
            base_path: String::from("my-test-path"),
            node_base_url: mockito::server_url(),
        }
    }

    #[test]
    fn test_new() {
        let expected_base_path = "/tmp/my/test/path";
        let manager = Manager::new(String::from(expected_base_path));
        assert_eq!(expected_base_path, manager.base_path);
    }

    #[test]
    fn test_get_latest() -> Result<(), Box<Error>> {
        let _mock_server = mock("GET", "/dist/index.json")
            .with_body_from_file("tests/node_test.json")
            .create();
        let manager = create_manager();
        let latest = manager.get_latest()?;
        assert_eq!(latest, "v12.7.0");
        Ok(())
    }

    #[test]
    fn test_get_lts() -> Result<(), Box<Error>> {
        let _mock_server = mock("GET", "/dist/index.json")
            .with_body_from_file("tests/node_test.json")
            .create();
        let manager = create_manager();
        let lts = manager.get_lts()?;
        assert_eq!(lts, "v10.16.0");
        Ok(())
    }
}
