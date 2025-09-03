#![doc = include_str!("../README.md")]
pub mod errors;
#[cfg(feature = "parser")]
pub mod parser;
pub mod solver;
pub use solver::*;
