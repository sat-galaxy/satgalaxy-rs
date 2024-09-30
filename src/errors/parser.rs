

use thiserror::Error;

use crate::parser::Rule;

#[derive(Error, Debug)]
pub enum ParserError{
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse CNF: {0}")]
    CnfParseError(#[from] pest::error::Error<Rule>),
    #[error("Number of variables ({0}) exceeds expected maximum ({1})")]
    TooManyVariables(i32, i32),
    #[error("Failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError)
}


