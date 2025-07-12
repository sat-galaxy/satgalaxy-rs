#[cfg(feature = "parser")]
mod parser;
use std::error::Error;

#[cfg(feature = "parser")]
pub use parser::ParserError;

#[derive(Debug)]
pub struct  SolverError(pub &'static str);

impl std::fmt::Display for SolverError   {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SolverError {
    fn description(&self) -> &str {
        self.0
    }
}
