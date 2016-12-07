use app_dirs::AppDirsError;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PError {
    AppDirsError(AppDirsError),
    CommandNotImplemented(String),
    Io(io::Error),
    MissingPrown(PathBuf),
    MissingCommand(String, PathBuf),
    MissingProjectList,
    PatternError(::glob::PatternError),
    NotATable(String),
}

impl From<io::Error> for PError {
    fn from(err: io::Error) -> PError {
        PError::Io(err)
    }
}

impl From<::glob::PatternError> for PError {
    fn from(err: ::glob::PatternError) -> PError {
        PError::PatternError(err)
    }
}

impl From<AppDirsError> for PError {
    fn from(err: AppDirsError) -> PError {
        PError::AppDirsError(err)
    }
}

impl fmt::Display for PError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PError::AppDirsError(ref e) => write!(fmt, "{}", e),
            &PError::CommandNotImplemented(ref s) => {
                write!(fmt, "The command {} is not implemented in prown", s)
            }
            &PError::Io(ref e) => write!(fmt, "{}", e),
            &PError::MissingPrown(ref p) => {
                write!(fmt, "There is no .prown.toml file in {}", p.display())
            }
            &PError::MissingCommand(ref s, ref p) => {
                write!(fmt,
                       "There is no {0} command in {1}\nAdd \'{0} = \"<command>\"\' in {1} under \
                        [commands] section",
                       s,
                       p.display())
            }
            &PError::MissingProjectList => write!(fmt, "There is no project list for now"),
            &PError::PatternError(ref p) => write!(fmt, "{}", p),
            &PError::NotATable(ref s) => {
                write!(fmt, "{0} is not a table, replace it with [{0}]", s)
            }
        }
    }
}

impl Error for PError {
    fn description(&self) -> &str {
        "TODO"
    }
}

pub type Result<T> = ::std::result::Result<T, PError>;
