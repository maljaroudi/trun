mod prompt;
use serde::Deserialize;

use std::io::BufRead;
#[derive(Deserialize)]
struct Content {
    prompt: Option<Vec<Prompt>>,
}

pub use prompt::Prompt;

pub fn interpret<T: BufRead>(buffer: &mut T) {
    let mut ret = String::new();
    buffer
        .read_to_string(&mut ret)
        .expect("Make sure the file exist and the format is correct");
    let tomlized: Content = toml::from_str(&ret).unwrap();
    if let Some(mut p) = tomlized.prompt {
        p.iter_mut().for_each(|v| v.run().unwrap_or(()));
    }
}
