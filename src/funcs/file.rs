use super::Runner;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use xxhash_rust::xxh3::xxh3_64;
#[derive(Deserialize)]
pub struct TFile {
    name: String,
    check: Option<bool>,
    src: PathBuf,
    dest: Option<PathBuf>,
    permissions: Option<u32>,
    exists: Option<bool>,
    #[serde(rename = "move")]
    moove: Option<bool>,
}

impl Runner for TFile {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("=================================================");
        println!("TASK {}", self.name);
        let src = Path::new(&self.src);
        if let Some(exists) = self.exists {
            if !exists {
                if src.exists() {
                    std::fs::remove_file(&src)?;
                    println!("REMOVING FILE ...");
                } else {
                    println!("FILE DOES NOT EXIST. CONTINUING ...");
                }
                if self.dest.is_some() {
                    println!("YOU ASKED TO REMOVE A FILE, A DESTINATION IS NOT NEEDED");
                }
                if self.moove.is_some() {
                    println!("MOVE AND EXISTS CAN'T COEXIST, IGNORING MOVE");
                }
                if self.permissions.is_some() {
                    println!("EXIST CAN'T COEXIST WITH PERMISSIONS");
                }
                if self.check.is_some() {
                    println!("EXIST AND CHECK DOES NOT NEED TO COEXIST");
                }
                println!("=================================================");
                return Ok(());
            }
        }
        if self.dest.is_none() {
            println!("YOU DID NOT PROVIDE A DESTINATION");
            println!("=================================================");
            return Ok(());
        }
        let dest_unwraped = self.dest.as_ref().unwrap();
        let dest = Path::new(&dest_unwraped);
        if self.check.unwrap_or(false) {
            if dest.exists() && src.exists() {
                let mut d_open = BufReader::new(File::open(dest)?);
                let mut d_bytes = vec![];
                d_open.read_to_end(&mut d_bytes)?;
                let mut s_open = BufReader::new(File::open(src)?);
                let mut s_bytes = vec![];
                s_open.read_to_end(&mut s_bytes)?;
                if xxh3_64(&d_bytes) == xxh3_64(&s_bytes) {
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
        if let Some(perm) = self.permissions {
            println!("Setting Permissions");
            std::fs::set_permissions(
                dest,
                std::fs::Permissions::from_mode(
                    u32::from_str_radix(&format!("{perm}"), 8).unwrap(),
                ),
            )?;
        }
        if self.moove.unwrap_or(false) {
            std::fs::remove_file(src)?;
        }

        println!("=================================================");
        Ok(())
    }
}
