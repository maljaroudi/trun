use super::{interpret, runner::Runner, TError};
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
#[derive(Deserialize)]
struct Recipe {
    name: String,
    #[serde(flatten)]
    steps: IndexMap<String, String>,
    panic: bool,
    skip: Option<bool>,
}

#[typetag::deserialize(name = "Recipe")]
impl Runner for Recipe {
    fn run(&mut self) -> Result<(), TError> {
        println!("STARTING TRUN RECIPE: {}", self.name);
        let mut current_dir = std::env::current_dir()?;
        current_dir.push("recipes");
        for t in self.steps.values() {
            let mut current_t = current_dir.clone();
            current_t.push(&t);
            let f = File::open(current_t)?;
            println!("RUNNING {t}");
            let mut buffer = BufReader::new(f);
            interpret(&mut buffer)?;
        }
        Ok(())
    }
}
