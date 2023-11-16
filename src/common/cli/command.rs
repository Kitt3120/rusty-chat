pub trait Action {
    fn run(&self, args: &[&str]) -> Result<(), String>;
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    action: Box<dyn Action>,
}

impl Command {
    pub fn new(name: &str, description: &str, usage: &str, action: Box<dyn Action>) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            usage: String::from(usage),
            action,
        }
    }

    pub fn run(&self, args: &[&str]) -> Result<(), String> {
        self.action.run(args)
    }
}

//TODO: Integration tests
