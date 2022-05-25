use core::str::Utf8Error;


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
impl From<DbusError> for TError {
    fn from(e: DbusError) -> Self {
        Self::DbusError(e)
    }
}

#[typetag::deserialize(tag = "module")]
pub trait Runner {
    fn run(&mut self) -> Result<(), TError>;
}
