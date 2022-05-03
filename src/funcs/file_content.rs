use super::Runner;
use serde::Deserialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::path::PathBuf;
#[derive(Deserialize)]
pub struct LineInFile {
    name: String,
    line: String,
    file: PathBuf,
}
#[typetag::deserialize]
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
        let mut buf_writer = BufWriter::new(opened);
        buf_writer.write_all(self.line.as_bytes())?;
        println!("FINISHED WRITING");
        println!("=================================================");
        Ok(())
    }
}
#[derive(Deserialize)]
struct BlockInFile {
    name: String,
    block: String,
    file: PathBuf,
    signature: String,
    comment: String,
}

#[typetag::deserialize]
impl Runner for BlockInFile {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let spattern = format!("{} ~!STARTING TRUN {}", &self.comment, &self.signature);
        let epattern = format!("{} !~ENDING TRUN {}", &self.comment, &self.signature);
        let source_file = Path::new(&self.file);
        if !source_file.exists() {
            let mut created = File::create(&self.file)?;
            writeln!(created, "{}", &spattern)?;
            writeln!(created)?;
            writeln!(created, "{}", &self.block)?;
            writeln!(created)?;
            writeln!(created, "{}", &epattern)?;
            println!("=================================================");
            return Ok(());
        }

        let mut opened = OpenOptions::new().read(true).write(true).open(&self.file)?;
        let mut buffer = BufReader::new(&opened);
        let mut stringed_buffer = String::new();
        buffer.read_to_string(&mut stringed_buffer)?;
        let s_index = stringed_buffer.find(&spattern);
        if let Some(start) = s_index {
            let e_index = stringed_buffer.find(&epattern);
            if let Some(end) = e_index {
                if self.block == stringed_buffer[start + spattern.len()..end].trim() {
                    println!("BLOCK FOUND");
                    println!("=================================================");
                    return Ok(());
                }
                println!("BLOCK DOESN'T MATCH");

                opened.seek(SeekFrom::Start(start as u64))?;
                opened.write_all(&b" ".repeat(end + epattern.len() - (start)))?;
                opened.seek(SeekFrom::Start((start) as u64))?;
                writeln!(opened, "{}", &spattern)?;
                writeln!(opened)?;
                writeln!(opened, "{}", &self.block)?;
                writeln!(opened)?;
                writeln!(opened, "{}", &epattern)?;
                //opened.write(&self.block.as_bytes())?;
                println!("FINISHED WRITING");
                println!("=================================================");
                return Ok(());
                // Block Doesn't Match, change it
            }
        }
        // let mut buf_writer = BufWriter::new(opened);
        // buf_writer.write_all(self.block.as_bytes())?;
        // println!("FINISHED WRITING");
        // println!("=================================================");
        Ok(())
    }
}
