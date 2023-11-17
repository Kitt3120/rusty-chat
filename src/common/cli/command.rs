pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn usage(&self) -> &str;
    fn run(&self, args: &[&str]) -> Result<(), String>;
}

//TODO: Integration tests
