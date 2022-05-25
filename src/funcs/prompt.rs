use super::runner::Runner;
use super::runner::TError;
use serde::Deserialize;
use std::io;
use std::process::Command;
#[derive(Deserialize)]
pub struct Prompt {
    name: String,
    strict: bool,
    command: String,
    message: String,
    answer: (String, String),
}

#[typetag::deserialize]
impl Runner for Prompt {
    fn run(&mut self) -> Result<(), TError> {
        println!("=================================================");
        println!("TASK {}", self.name);
        self.answer = (
            self.answer.0.trim().to_owned(),
            self.answer.1.trim().to_owned(),
        );
        let mut buffer = String::new();
        if !self.strict {
            self.answer.0 = self.answer.0.to_lowercase();
            self.answer.1 = self.answer.1.to_lowercase();
        }
        while buffer.trim() != self.answer.0 && buffer.trim() != self.answer.1 {
            println!("{} [{}/{}]", self.message, self.answer.0, self.answer.1);
            buffer.clear();
            if self.strict {
                io::stdin().read_line(&mut buffer)?;
                buffer = buffer.trim().to_owned();
            } else {
                io::stdin().read_line(&mut buffer)?;
                buffer = buffer.trim().to_lowercase();
            }
        }
        if self.strict {
            if buffer == self.answer.1 {
                println!("=================================================");
                return Ok(());
            }
        } else if buffer == self.answer.1.to_lowercase() {
            println!("=================================================");
            return Ok(());
        }
        let args = self.command.split_whitespace().collect::<Vec<&str>>();
        if args.len() == 1 {
            Command::new(args[0]).spawn()?.wait_with_output()?;
            println!("=================================================");
            return Ok(());
        }
        Command::new(args[0])
            .args(&args[1..])
            .spawn()?
            .wait_with_output()?;

        println!("=================================================");
        Ok(())
    }
}
