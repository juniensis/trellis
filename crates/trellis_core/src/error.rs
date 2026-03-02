use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Unspecified,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "CoreError: Unspecified"),
        }
    }
}

impl std::error::Error for Error {}
