use std::io::stdin;

use crate::common::{cli::command::Command, threading::CancellationToken};

pub struct CommandHandler {
    commands: Vec<Command>,
}

impl CommandHandler {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|command| command.name == name)
    }

    pub fn handle(&self, command: &str, args: &[&str]) -> Result<(), String> {
        let command = match self.get_command(command) {
            Some(command) => command,
            None => return Err(format!("Unknown command: {}", command)),
        };

        let run_result = command.run(args);

        match run_result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn handle_stdin(&self, cancellation_token: CancellationToken) -> Result<(), String> {
        let mut buffer = String::new();

        loop {
            let cancelled = match cancellation_token.is_cancelled() {
                Ok(cancelled) => cancelled,
                Err(err) => return Err(format!("Failed to check cancellation token: {}", err)),
            };

            if cancelled {
                return Ok(());
            }

            buffer.clear(); // Clear buffer before reading from stdin instead of at the end of the loop to avoid borrow checker issues
            let read_result = stdin().read_line(&mut buffer);
            if let Err(err) = read_result {
                return Err(format!("Failed to read from stdin: {}", err));
            }

            let input = buffer.trim();
            if input.is_empty() {
                continue;
            }

            let split: Vec<&str> = input.split_whitespace().collect();
            let command_name = split.first().unwrap();
            let args = &split[1..];

            let command = match self.get_command(command_name) {
                Some(command) => command,
                None => {
                    println!("Unknown command: {}", command_name);
                    continue;
                }
            };

            command.run(args)?;
        }
    }
}

//TODO: Integration tests
