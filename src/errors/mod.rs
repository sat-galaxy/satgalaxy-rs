#[cfg(feature = "parser")]
mod parser;
use std::error::Error;

#[cfg(feature = "parser")]
pub use parser::ParserError;

#[derive(Debug)]
pub struct  OptionError(pub &'static str);

impl std::fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for OptionError {
    fn description(&self) -> &str {
        self.0
    }
}
