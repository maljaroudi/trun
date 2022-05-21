use super::runner::TError;
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
    fn run(&mut self) -> Result<(), TError> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let source_file = Path::new(&self.file);
        if !source_file.exists() {
            let mut created = File::create(&self.file).map_err(TError::FileError)?;
            created
                .write_all(self.line.as_bytes())
                .map_err(TError::FileError)?;
            println!("=================================================");
            return Ok(());
        }

        let opened = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&self.file)
            .map_err(TError::FileError)?;
        let buffer = BufReader::new(&opened);
        if buffer.lines().any(|x| x.unwrap() == self.line) {
            println!("LINE EXISTS");
            println!("=================================================");
            return Ok(());
        }
        let mut buf_writer = BufWriter::new(opened);
        buf_writer
            .write_all(self.line.as_bytes())
            .map_err(TError::FileError)?;
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
    fn run(&mut self) -> Result<(), TError> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let spattern = format!("{} ~!STARTING TRUN {}", &self.comment, &self.signature);
        let epattern = format!("{} !~ENDING TRUN {}", &self.comment, &self.signature);
        let source_file = Path::new(&self.file);
        if !source_file.exists() {
            let mut created = File::create(&self.file).map_err(TError::FileError)?;
            writeln!(created, "{}", &spattern).map_err(TError::FileError)?;
            writeln!(created).map_err(TError::FileError)?;
            writeln!(created, "{}", &self.block).map_err(TError::FileError)?;
            writeln!(created).map_err(TError::FileError)?;
            writeln!(created, "{}", &epattern).map_err(TError::FileError)?;
            println!("FINISHED WRITING");
            println!("=================================================");
            return Ok(());
        }

        let mut opened = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.file)
            .map_err(TError::FileError)?;
        let mut buffer = BufReader::new(&opened);
        let mut stringed_buffer = String::new();
        buffer
            .read_to_string(&mut stringed_buffer)
            .map_err(TError::FileError)?;
        let s_index = stringed_buffer.find(&spattern);
        if let Some(start) = s_index {
            let e_index = stringed_buffer.find(&epattern);
            if let Some(end) = e_index {
                if self.block.trim() == stringed_buffer[start + spattern.len()..end].trim() {
                    println!("BLOCK FOUND");
                    println!("=================================================");
                    return Ok(());
                }
                println!("BLOCK DOESN'T MATCH REPAIRING ..");

                opened
                    .seek(SeekFrom::Start(start as u64))
                    .map_err(TError::SeekError)?;
                write!(opened, "{}", &" ".repeat(end + epattern.len() - (start)))
                    .map_err(TError::SeekError)?;
                opened
                    .seek(SeekFrom::Start((start) as u64))
                    .map_err(TError::SeekError)?;
                writeln!(opened, "{}", &spattern).map_err(TError::FileError)?;
                writeln!(opened).map_err(TError::FileError)?;
                writeln!(opened, "{}", &self.block).map_err(TError::FileError)?;
                writeln!(opened).map_err(TError::FileError)?;
                writeln!(opened, "{}", &epattern).map_err(TError::FileError)?;
                //opened.write(&self.block.as_bytes())?;
                println!("FINISHED WRITING");
                println!("=================================================");
                return Ok(());
                // Block Doesn't Match, change it
            }
        }
        // let mut buf_writer = BufWriter::new(opened);
        opened.seek(SeekFrom::End(0)).map_err(TError::FileError)?;
        writeln!(opened, "{}", &spattern).map_err(TError::FileError)?;
        writeln!(opened).map_err(TError::FileError)?;
        writeln!(opened, "{}", &self.block).map_err(TError::FileError)?;
        writeln!(opened).map_err(TError::FileError)?;
        writeln!(opened, "{}", &epattern).map_err(TError::FileError)?;
        //buf_writer.write_all(self.block.as_bytes())?;
        println!("FINISHED WRITING");
        println!("=================================================");
        Ok(())
    }
}
