use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StravaAuthError {
    pub code: i8,
    pub message: String,
}

impl fmt::Display for StravaAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}. {}", self.code, self.message)
    }
}

impl Error for StravaAuthError {}
