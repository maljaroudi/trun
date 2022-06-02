use super::opts::Opts;
use super::runner::Runner;
use super::runner::TError;
use serde::Deserialize;
use std::io;
use std::process::Command;
#[derive(Deserialize)]
#[serde(rename = "Command")]
pub struct Cmd {
    name: String,
    strict: Option<bool>,
    command: String,
    message: String,
    answer: Option<(String, String)>,
    #[serde(flatten)]
    opts: Opts,
}

impl Runner for Cmd {
    fn run(&mut self) -> Result<(), TError> {
        println!("=================================================");
        println!("TASK {}", self.name);
        if let Some((ref mut answer0, ref mut answer1)) = self.answer {
            *answer0 = answer0.trim().to_owned();
            *answer1 = answer1.trim().to_owned();
            let mut buffer = String::new();
            if !self.strict.unwrap_or(false) {
                *answer0 = answer0.to_lowercase();
                *answer1 = answer1.to_lowercase();
            }
            while buffer.trim() != answer0 && buffer.trim() != answer1 {
                println!("{} [{}/{}]", self.message, answer0, answer1);
                buffer.clear();
                if self.strict.unwrap_or(false) {
                    io::stdin().read_line(&mut buffer)?;
                    buffer = buffer.trim().to_owned();
                } else {
                    io::stdin().read_line(&mut buffer)?;
                    buffer = buffer.trim().to_lowercase();
                }
            }
            if self.strict.unwrap_or(false) {
                if buffer == *answer1 {
                    println!("=================================================");
                    return Ok(());
                }
            } else if buffer == answer1.to_lowercase() {
                println!("=================================================");
                return Ok(());
            }
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
    fn panics(&self) -> bool {
        if let Some(x) = self.opts.panics {
            return x;
        }
        true
    }
}
