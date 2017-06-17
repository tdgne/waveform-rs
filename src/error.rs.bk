use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidSizeError {
    pub var_name: String,
}

impl fmt::Display for InvalidSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid size of {}", self.var_name)
    }
}
impl Error for InvalidSizeError {
    fn description(&self) -> &str {
        "An numeric value of an invalid size has been passed."
    }
}


