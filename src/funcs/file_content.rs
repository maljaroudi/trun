use super::Runner;
use serde::Deserialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct LineInFile {
    name: String,
    line: String,
    file: PathBuf,
}

impl Runner for LineInFile {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let source_file = Path::new(&self.file);
        if !source_file.exists() {
            let mut created = File::create(&self.file)?;
            created.write_all(self.line.as_bytes())?;
            println!("=================================================");
            return Ok(());
        }

        let opened = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&self.file)?;
        let buffer = BufReader::new(&opened);
        if buffer.lines().any(|x| x.unwrap() == self.line) {
            println!("LINE EXISTS");
            println!("=================================================");
            return Ok(());
        }
        println!("HERE");
        let mut buf_writer = BufWriter::new(opened);
        buf_writer.write_all(self.line.as_bytes())?;
        println!("HERE2");
        Ok(())
    }
}

struct BlockInFile {
    name: String,
    block: String,
    file: PathBuf,
    signature: String,
    comment: String,
}
