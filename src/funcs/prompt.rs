use serde::{Deserialize, Serialize};
use std::io;
use std::process::Command;
pub use toml::*;
#[derive(Deserialize, Serialize)]
pub struct Prompt {
    strict: bool,
    command: String,
    message: String,
    answer: (String, String),
}

impl Prompt {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("{} [{}/{}]", self.message, self.answer.0, self.answer.1);
        let mut buffer = String::new();
        if !self.strict {
            self.answer.0 = self.answer.0.to_lowercase();
            self.answer.1 = self.answer.1.to_lowercase();
        }
        while buffer.trim() != self.answer.0.trim() && buffer != self.answer.1 {
            if self.strict {
                io::stdin().read_line(&mut buffer)?;
            } else {
                io::stdin().read_line(&mut buffer)?;
                buffer = buffer.to_lowercase();
            }
        }
        if self.strict {
            if buffer == self.answer.1 {
                return Ok(());
            }
        } else if buffer == self.answer.1.to_lowercase() {
            return Ok(());
        }
        let args = self.command.split_whitespace().collect::<Vec<&str>>();
        if args.len() == 1 {
            Command::new(args[0]).spawn()?.wait_with_output()?;
            return Ok(());
        }
        Command::new(args[0]).args(&args[1..]).spawn()?.wait_with_output()?;

        Ok(())
    }
}
