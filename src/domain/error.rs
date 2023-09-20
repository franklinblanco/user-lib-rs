use std::fmt::Display;


/// Used to return a simple error from FromStr implementations
#[derive(Debug)]
pub struct FromStrError;

impl Display for FromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing string into value. FromStrError.")
    }
}

impl std::error::Error for FromStrError {}