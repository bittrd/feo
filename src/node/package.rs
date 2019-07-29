use reqwest;
use std::error::Error;

pub struct Package {
    package_name: String,
    registry_url: String,
    tag_or_version: String,
}
impl Package {
    pub fn new(package_string: String) -> Package {
        let tag_or_version = get_version(&package_string);
        let package_name = get_name(&package_string);

        Package {
            package_name: package_name,
            registry_url: String::from("http://registry.npmjs.com"),
            tag_or_version: tag_or_version,
        }
    }
    pub fn get_node_range(&self) -> Result<String, Box<Error>> {
        let url = format!("{}/{}", self.registry_url, self.package_name);
        let resp: serde_json::Value = reqwest::get(&url)?.json()?;
        println!("{}", resp.to_string());
        Ok(String::from(""))
    }
}

fn get_name(package_string: &String) -> String {
    let at_pos = package_string.rfind("@");
    match at_pos {
        Some(pos) => {
            if pos != 0 {
                return package_string.get(..pos).unwrap().to_string();
            }
            package_string.to_string()
        }
        None => package_string.to_string(),
    }
}

fn get_version(package_string: &String) -> String {
    let at_pos = package_string.rfind("@");
    match at_pos {
        Some(pos) => {
            if pos != 0 {
                return package_string
                    .get(pos + 1..)
                    .unwrap_or("latest")
                    .to_string();
            }
            String::from("latest")
        }
        None => String::from("latest"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_package() -> Package {
        let mut pkg = Package::new(String::from("@bittrd/hello-cli"));
        pkg.registry_url = mockito::server_url();
        return pkg;
    }
    fn create_package_with_name(package_name: String) -> Package {
        let mut pkg = Package::new(package_name);
        pkg.registry_url = mockito::server_url();
        return pkg;
    }
    #[test]
    fn test_new() -> Result<(), Box<Error>> {
        let package = Package::new(String::from("hello-cli"));
        assert_eq!(package.package_name, "hello-cli");
        assert_eq!(package.tag_or_version, "latest");
        Ok(())
    }
    #[test]
    fn test_new_with_version() -> Result<(), Box<Error>> {
        let package = Package::new(String::from("hello-cli@2.0.3"));
        assert_eq!(package.package_name, "hello-cli");
        assert_eq!(package.tag_or_version, "2.0.3");
        Ok(())
    }
    #[test]
    fn test_new_with_scope_without_version() -> Result<(), Box<Error>> {
        let package = Package::new(String::from("@bittrd/hello-cli"));
        assert_eq!(package.package_name, "@bittrd/hello-cli");
        assert_eq!(package.tag_or_version, "latest");
        Ok(())
    }
    #[test]
    fn test_new_with_scope_and_version() -> Result<(), Box<Error>> {
        let package = Package::new(String::from("@bittrd/hello-cli@2.0.3"));
        assert_eq!(package.tag_or_version, "2.0.3");
        assert_eq!(package.package_name, "@bittrd/hello-cli");
        Ok(())
    }
    #[test]
    fn test_get_node_range() -> Result<(), Box<Error>> {
        let _mock = mockito::mock("GET", "/@bittrd/hello-cli")
            .with_body_from_file("tests/npm_registry_package.json")
            .create();
        let package = create_package();
        let node_range = package.get_node_range()?;
        assert_eq!(node_range, "^8.16.0 || ^10.16.0");
        Ok(())
    }
    #[test]
    fn test_get_node_range_specific_version() -> Result<(), Box<Error>> {
        let _mock = mockito::mock("GET", "/@bittrd/hello-cli")
            .with_body_from_file("tests/npm_registry_package.json")
            .create();
        let package = create_package_with_name(String::from("@bittrd/hello-cli@2.0.3"));
        let node_range = package.get_node_range()?;
        assert_eq!(node_range, "");
        Ok(())
    }
}
