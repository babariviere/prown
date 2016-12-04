use std::error;
use std::fmt;
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

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::CommandNotImplemented(ref s) => {
                write!(fmt, "The command {} is not implemented in prown", s)
            }
            &Error::Io(ref e) => write!(fmt, "{}", e),
            &Error::MissingPrown(ref p) => {
                write!(fmt, "There is no .prown.toml file in {}", p.display())
            }
            &Error::MissingCommand(ref s, ref p) => {
                write!(fmt,
                       "There is no {0} command in {1}\nAdd \'{0} = \"<command>\"\' in {1} under \
                        [commands] section",
                       s,
                       p.display())
            }
            &Error::PatternError(ref p) => write!(fmt, "{}", p),
            &Error::NotATable(ref s) => write!(fmt, "{0} is not a table, replace it with [{0}]", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "TODO"
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
