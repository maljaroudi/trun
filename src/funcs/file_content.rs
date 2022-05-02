use super::Runner;
use std::path::PathBuf;
struct LineInFile {
    line: String,
    file: PathBuf,
}

impl Runner for LineInFile {
    fn run(&mut self) -> Result<(), std::io::Error> {
    todo!()
    }
}

struct BlockInFile {
    block: String,
    file: PathBuf,
}
