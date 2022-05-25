use super::runner::Runner;
use super::runner::TError;
use serde::Deserialize;
use std::process::Command;
#[derive(Deserialize)]
pub struct Loop {
    name: String,
    command: String,
    iterations: usize,
    start: Option<usize>,
}
#[typetag::deserialize]
impl Runner for Loop {
    fn run(&mut self) -> Result<(), TError> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let args = self.command.split_whitespace().collect::<Vec<&str>>();
        if args.len() == 1 {
            for _ in self.start.unwrap_or_default()..self.iterations {
                Command::new(args[0]).spawn()?.wait_with_output()?;
            }
            println!("=================================================");
            return Ok(());
        }
        for _ in self.start.unwrap_or_default()..self.iterations {
            Command::new(args[0])
                .args(&args[1..])
                .spawn()?
                .wait_with_output()?;
        }
        println!("=================================================");
        Ok(())
    }
}
