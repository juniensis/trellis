use std::{error::Error, fmt::Display};

pub type UiResult<T> = Result<T, UiError>;
#[derive(Debug)]
pub enum UiError {
    BufferDimensionMismatch((usize, usize), (usize, usize)),
    Unspecified,
}

impl Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferDimensionMismatch((w, x), (y, z)) => write!(
                f,
                "UiError: Attempted operation between buffers of mismatched size. {}x{} != {}x{}",
                w, x, y, z
            ),
            Self::Unspecified => write!(f, "UiError: Unspecified"),
        }
    }
}

impl Error for UiError {}
