use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum TerminalError {
    IoError(io::Error),
    Undefined,
}

pub type TerminalResult<T> = Result<T, TerminalError>;

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{e}"),
            Self::Undefined => writeln!(f, "TerminalError: Unknown/unhandled error occured."),
        }
    }
}

impl Error for TerminalError {}

impl From<io::Error> for TerminalError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
