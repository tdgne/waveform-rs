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


#[derive(Debug)]
pub struct InconsistentFormatError;

impl fmt::Display for InconsistentFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Color formats of background and foreground must be consistent."
        )
    }
}
impl Error for InconsistentFormatError {
    fn description(&self) -> &str {
        "Color formats of background and foreground must be consistent."
    }
}
