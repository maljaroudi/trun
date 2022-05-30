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
    skip: Option<bool>,
    // Opts is manually flattened here due to a bug in serde (probably) with IndexMap/HashMaps
    // serde will conflict between the type. Making these fields as Strings but expecting a bool
    // when changed to a string.
    panics: Option<bool>,
    debug: Option<bool>,
}

#[typetag::deserialize(name = "Recipe")]
impl Runner for Recipe {
    fn run(&mut self) -> Result<(), TError> {
        println!("STARTING TRUN RECIPE: {}", self.name);
        let mut current_dir = std::env::current_dir()?;
        current_dir.push("recipes");
        for t in self.steps.values() {
            if self.skip.unwrap_or(false) {
                continue;
            }
            let mut current_t = current_dir.clone();
            current_t.push(&t);
            let f = File::open(current_t)?;
            println!("RUNNING {t}");
            let mut buffer = BufReader::new(f);
            let nested_panic = self.panics();
            interpret(&mut buffer, nested_panic)?;
        }
        Ok(())
    }
    // TODO: We should take this and replace all recipes inside with this flag
    fn panics(&self) -> bool {
        if let Some(x) = self.panics {
            return x;
        }
        true
    }
}
