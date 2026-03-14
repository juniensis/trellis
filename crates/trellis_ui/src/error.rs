use std::fmt::Display;

use trellis_terminal::error::TerminalError;

#[derive(Debug)]
pub enum Error {
    TerminalError(TerminalError),
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TerminalError(e) => write!(f, "{e}"),
            Self::Unknown => write!(f, "UiError: Unknown"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl From<TerminalError> for Error {
    fn from(value: TerminalError) -> Self {
        Self::TerminalError(value)
    }
}
