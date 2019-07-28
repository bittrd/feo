pub struct Command {}
impl Command {
    pub fn run(&self, package: String) {
        println!("Installing package: {}", package);
    }
}
