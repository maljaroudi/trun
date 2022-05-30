use core::str::Utf8Error;
use std::io::ErrorKind;
#[derive(Debug)]
pub enum TError {
    IOError(std::io::Error),
    Utf8Error(Utf8Error),
    TomlError(toml::de::Error),
    #[cfg(target_os = "linux")]
    DbusError(dbus::Error),
}

impl From<std::io::Error> for TError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == ErrorKind::NotFound {
            eprintln!("Could Not Find Apt. This Module Cannot be Executed Without Apt");
        }
        Self::IOError(e)
    }
}

impl From<Utf8Error> for TError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

impl From<toml::de::Error> for TError {
    fn from(e: toml::de::Error) -> Self {
        Self::TomlError(e)
    }
}

#[cfg(target_os = "linux")]
impl From<dbus::Error> for TError {
    fn from(e: dbus::Error) -> Self {
        Self::DbusError(e)
    }
}

#[typetag::deserialize(tag = "module")]
pub trait Runner {
    fn run(&mut self) -> Result<(), TError>;
    fn panics(&mut self) -> bool {
        true
    }
}
