mod looop;
mod file;
mod file_content;
mod prompt;
mod runner;
use runner::Runner;
use indexmap::IndexMap;
use std::io::BufRead;
//#[derive(Deserialize)]
//struct Content(Vec<Box<dyn  Runner>>);

pub fn interpret<T: BufRead>(buffer: &mut T) -> Result<(), toml::de::Error> {
    let mut ret = String::new();
    buffer
        .read_to_string(&mut ret)
        .expect("Make sure the file exist and the format is correct");
    let mut tomlized: IndexMap<String,Box<dyn Runner>> =  toml::from_str(&ret)?;
    for runner in tomlized.values_mut(){
            runner.run().unwrap_or(());
        }
   Ok(())
}
