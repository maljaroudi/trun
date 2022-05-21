use core::str::Utf8Error;

pub enum TError {
    IOError(std::io::Error),
    CmdError(std::io::Error),
    AptError(std::io::Error),
    FileError(std::io::Error),
    SeekError(std::io::Error),
    Utf8Error(Utf8Error),

    #[cfg(target_os = "linux")]
    DbusError(dbus::Error),
}

#[typetag::deserialize(tag = "module")]
pub trait Runner {
    fn run(&mut self) -> Result<(), TError>;
}
