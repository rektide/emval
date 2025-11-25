#[cfg(feature = "python")]
use pyo3::exceptions::{PySyntaxError, PyValueError};
#[cfg(feature = "python")]
use pyo3::prelude::*;

/// An error enum for email validation.
#[derive(Debug)]
pub enum ValidationError {
    /// A syntax error.
    SyntaxError(String),
    /// An error involving some input value.
    ValueError(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),
            ValidationError::ValueError(msg) => write!(f, "ValueError: {}", msg),
        }
    }
}

#[cfg(feature = "python")]
impl From<ValidationError> for PyErr {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::SyntaxError(msg) => PySyntaxError::new_err(msg),
            ValidationError::ValueError(msg) => PyValueError::new_err(msg),
        }
    }
}
