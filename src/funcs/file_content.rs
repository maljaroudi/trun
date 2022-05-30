use super::opts::Opts;
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
    #[serde(flatten)]
    opts: Opts,
}

impl Runner for LineInFile {
    fn run(&mut self) -> Result<(), TError> {
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
    fn panics(&self) -> bool {
        if let Some(x) = self.opts.panics {
            return x;
        }
        true
    }
}
#[derive(Deserialize)]
pub struct BlockInFile {
    name: String,
    block: String,
    file: PathBuf,
    signature: String,
    comment: String,
    #[serde(flatten)]
    opts: Opts,
}

impl Runner for BlockInFile {
    fn run(&mut self) -> Result<(), TError> {
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
            println!("FINISHED WRITING");
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
                if self.block.trim() == stringed_buffer[start + spattern.len()..end].trim() {
                    println!("BLOCK FOUND");
                    println!("=================================================");
                    return Ok(());
                }
                println!("BLOCK DOESN'T MATCH REPAIRING ..");

                opened.seek(SeekFrom::Start(start as u64))?;
                write!(opened, "{}", &" ".repeat(end + epattern.len() - (start)))?;
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
        opened.seek(SeekFrom::End(0))?;
        writeln!(opened, "{}", &spattern)?;
        writeln!(opened)?;
        writeln!(opened, "{}", &self.block)?;
        writeln!(opened)?;
        writeln!(opened, "{}", &epattern)?;
        //buf_writer.write_all(self.block.as_bytes())?;
        println!("FINISHED WRITING");
        println!("=================================================");
        Ok(())
    }

    fn panics(&self) -> bool {
        if let Some(x) = self.opts.panics {
            return x;
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn line_in_file() {
        let _ = std::fs::remove_file("test_line.txt");
        let mut f = LineInFile {
            name: "Test".to_owned(),
            line: "This IS a Test String".to_owned(),
            file: "test_line.txt".into(),
            opts: Default::default(),
        };
        f.run().unwrap();
        let mut ff = File::open("test_line.txt").unwrap();
        let mut stringer = String::new();
        ff.read_to_string(&mut stringer).unwrap();
        assert_eq!(stringer, "This IS a Test String");
    }
    #[test]
    fn block_in_file() {
        let _ = std::fs::remove_file("test_block.txt");
        let mut f = BlockInFile {
            name: "Test".to_owned(),
            block: "This is a test block for the block in file module,\nit should be able to detect the block in the file if programmed correctly".to_owned(),
            file: "test_block.txt".into(),
            comment: "//".to_owned(),
            signature: "TEST".to_owned(),
            opts: Default::default()
        };
        f.run().unwrap();
        let mut ff = File::open("test_block.txt").unwrap();
        let mut stringer = String::new();
        ff.read_to_string(&mut stringer).unwrap();

        assert_eq!(stringer,"// ~!STARTING TRUN TEST\n\nThis is a test block for the block in file module,\nit should be able to detect the block in the file if programmed correctly\n\n// !~ENDING TRUN TEST\n");
    }
}
