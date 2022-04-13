use super::Runner;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use twox_hash::xxh3::hash64;
#[derive(Deserialize)]
pub struct Copy {
    name: String,
    check: bool,
    src: PathBuf,
    dest: PathBuf,
}

impl Runner for Copy {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let dest = Path::new(&self.dest);
        let src = Path::new(&self.src);
        if self.check {
            if dest.exists() && src.exists() {
                let mut d_open = BufReader::new(File::open(dest)?);
                let mut d_bytes = vec![];
                d_open.read_to_end(&mut d_bytes)?;
                let mut s_open = BufReader::new(File::open(src)?);
                let mut s_bytes = vec![];
                s_open.read_to_end(&mut s_bytes)?;
                if hash64(&d_bytes) == hash64(&s_bytes) {
                    println!("BOTH FILES MATCHES");
                    println!("=================================================");
                    return Ok(());
                }
                println!("FILES DOES NOT MATCH");
            } else if !src.exists() {
                println!("INVALID SOURCE: FILE DOES NOT EXIST");
                println!("=================================================");
                return Ok(());
            }
        }
        println!("COPYING FILES...");
        std::fs::copy(&src, &dest)?;

        println!("=================================================");
        Ok(())
    }
}
