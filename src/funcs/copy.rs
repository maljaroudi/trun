use super::Runner;
use serde::Deserialize;
use sha256::digest_file;
use std::path::{Path, PathBuf};
#[derive(Deserialize)]
pub struct Copy {
    name: String,
    check: bool,
    src: PathBuf,
    dest: PathBuf,
}

impl Runner for Copy {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("TASK {}", self.name);
        let dest = Path::new(&self.dest);
        let src = Path::new(&self.src);
        if self.check {
            if dest.exists() && src.exists() {
                if digest_file(dest)? == digest_file(src)? {
                    println!("BOTH FILES MATCHES");
                    return Ok(());
                }
                println!("FILES DOES NOT MATCH");
                return Ok(());
            }
            println!("FILES DOES NOT EXIST, PROCEEDING");
        }
        println!("COPYING FILES...");
        std::fs::copy(&src, &dest)?;

        Ok(())
    }
}
