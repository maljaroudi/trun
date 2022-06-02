#[cfg(target_os = "linux")]
pub mod apt;
#[cfg(target_os = "linux")]
use apt::Apt;

mod command;
mod file;
mod file_content;
mod looop;
mod opts;
mod recipe;
pub mod runner;

#[cfg(target_os = "linux")]
mod systemd;
#[cfg(target_os = "linux")]
use systemd::Systemd;

use command::Cmd;
use enum_dispatch::enum_dispatch;
use file::TFile;
use file_content::{BlockInFile, LineInFile};
use indexmap::IndexMap;
use looop::Loop;
use recipe::Recipe;
use runner::{Runner, TError};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::io::BufRead;

#[enum_dispatch(Runner)]
#[derive(Deserialize)]
#[serde(tag = "module")]
enum Modules {
    #[cfg(target_os = "linux")]
    Apt,
    #[cfg(target_os = "linux")]
    Systemd,

    BlockInFile,
    LineInFile,
    #[serde(rename = "Command")]
    Cmd,
    #[serde(rename = "File")]
    TFile,
    Loop,
    Recipe,
}

#[derive(Deserialize)]
struct Content {
    #[serde(flatten)]
    modules: IndexMap<String, Modules>,
}

pub fn interpret<T: BufRead>(buffer: &mut T, nested_panic: bool) -> Result<(), TError> {
    let mut ret = String::new();
    buffer
        .read_to_string(&mut ret)
        .expect("Make sure the file exist and the format is correct");
    let mut tomlized: Content = toml::from_str(&ret).map_err(TError::TomlError)?;
    for runner in tomlized.modules.values_mut() {
        if runner.panics() || nested_panic {
            runner.run()?;
        } else {
            runner.run().unwrap_or(());
        }
    }
    Ok(())
}
