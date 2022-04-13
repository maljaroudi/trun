use super::runner::Runner;
use serde::Deserialize;
use std::process::Command;
#[derive(Deserialize)]
pub struct Loop {
    name: String,
    command: String,
    iterations: usize,
}

impl Runner for Loop {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("TASK {}", self.name);
        let args = self.command.split_whitespace().collect::<Vec<&str>>();
        if args.len() == 1 {
            for _ in 0..self.iterations {
                Command::new(args[0]).spawn()?.wait_with_output()?;
            }
            return Ok(());
        }
        for _ in 0..self.iterations {
            Command::new(args[0])
                .args(&args[1..])
                .spawn()?
                .wait_with_output()?;
        }
        Ok(())
    }
}
