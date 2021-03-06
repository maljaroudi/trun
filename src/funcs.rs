#[cfg(target_os = "linux")]
mod apt;

mod file;
mod file_content;
mod looop;
mod opts;
mod prompt;
mod recipe;
pub mod runner;

#[cfg(target_os = "linux")]
mod systemd;

use indexmap::IndexMap;
use runner::{Runner, TError};
use std::io::BufRead;
//#[derive(Deserialize)]
//struct Content(Vec<Box<dyn  Runner>>);

pub fn interpret<T: BufRead>(buffer: &mut T, nested_panic: bool) -> Result<(), TError> {
    let mut ret = String::new();
    buffer
        .read_to_string(&mut ret)
        .expect("Make sure the file exist and the format is correct");
    let mut tomlized: IndexMap<String, Box<dyn Runner>> =
        toml::from_str(&ret).map_err(TError::TomlError)?;
    for runner in tomlized.values_mut() {
        if runner.panics() || nested_panic {
            runner.run()?;
        } else {
            runner.run().unwrap_or(());
        }
    }
    Ok(())
}
