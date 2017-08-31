use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CommonError {
    text: &'static str,
}

impl CommonError {
    pub fn new(text: &'static str) -> CommonError {
        CommonError { text }
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        self.text
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
