use crate::common::{cli::Command, threading::CancellationTokenSource};

pub struct ExitCommand {
    cancellation_token_source: CancellationTokenSource,
}

impl ExitCommand {
    pub fn new(cancellation_token_source: CancellationTokenSource) -> Self {
        Self {
            cancellation_token_source,
        }
    }
}

impl Command for ExitCommand {
    fn name(&self) -> &str {
        "exit"
    }

    fn description(&self) -> &str {
        "Exits the program"
    }

    fn usage(&self) -> &str {
        "exit"
    }

    fn run(&self, _args: &[&str]) -> Result<(), String> {
        match self.cancellation_token_source.cancel() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to cancel CancellationTokenSource: {}", err)),
        }
    }
}
