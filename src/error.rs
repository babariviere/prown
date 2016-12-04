use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CommandNotImplemented(String),
    Io(io::Error),
    MissingPrown(PathBuf),
    MissingCommand(String, PathBuf),
    PatternError(::glob::PatternError),
    NotATable(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<::glob::PatternError> for Error {
    fn from(err: ::glob::PatternError) -> Error {
        Error::PatternError(err)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
