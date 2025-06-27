use std::{
    cmp::max,
    fs,
    io::{self, Read}, path::Path,
};

use crate::{errors::ParserError, parser::AsDimacs};

use pest::Parser;
#[derive(pest_derive::Parser)]
#[grammar = "../pest/dimacs.pest"]
struct DIMACSParser;

/// Parses a DIMACS CNF format string into a `CnfFormula` struct.
/// # Example
/// ```rust
///  use rssat::parser::parse_dimacs_cnf;
///
/// let dimacs_content = "c This is a comment
/// p cnf 3 2
/// 1 -3 0
/// ";
///     match parse_dimacs_cnf(dimacs_content,false) {
///         Ok(cnf) => {
///             assert_eq!(cnf.num_vars,3);
///             assert_eq!(cnf.num_clauses,2);
///         },
///         Err(e) => println!("Error: {:?}", e),
///    }
/// ```
///
/// # Usage
///  To use the `parse_dimacs_cnf`, ensure the `dimacs_` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["dimacs_"] }
/// ```
/// # Arguments
///
/// * `input` - A string slice that holds the content of the DIMACS CNF file.
/// * `strict` - A boolean flag that determines whether to enforce strict parsing rules.
///
/// # Returns
///
/// * `Ok(CnfFormula)` - If parsing is successful, returns a `CnfFormula` struct.
/// * `Err(ParserError)` - If parsing fails, returns a `ParserError`.
///
/// # Errors
///
/// This function will return an error if:
/// * The input does not conform to the DIMACS CNF format.
/// * The number of variables exceeds the declared number when in strict mode.
/// * Any integer parsing fails.
///
/// # Behavior
///
/// * Parses the input string according to DIMACS CNF format rules.
/// * In strict mode, it enforces the declared number of variables and clauses.
/// * Constructs a `CnfFormula` with parsed clauses and variable information.
pub fn parse_dimacs_cnf<D:AsDimacs>(input: &str, strict: bool,dim:&mut D) -> Result<(), ParserError> {
    let mut num_vars = 0;
    let mut variables = 0;
    let mut clauses = 0;
    let mut num_clauses = 0;
    let pairs = DIMACSParser::parse(Rule::file, input)?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::cluase => {
                    if strict  {
                        if clauses > 0 && num_clauses >= clauses {
                            return Err(ParserError::TooManyClauses(num_clauses, clauses));
                        }
                        if num_vars>0 && num_vars>=variables{
                            return Err(ParserError::TooManyVariables(num_vars, variables));
                        }
                    }

                    let mut clause = Vec::<i32>::new();
                    for lit_pair in inner_pair.into_inner() {
                        let lit = lit_pair.as_str().parse::<i32>()?;
                        let abs = lit.abs();
                        num_vars = max(abs, num_vars);
                        clause.push(lit);
                    }
                    num_clauses+=1;
                    dim.add_clause(clause);
                }
                Rule::def => {
                    for def_rule in inner_pair.into_inner() {
                        match def_rule.as_rule() {
                            Rule::variables => {
                                variables = def_rule.as_str().parse::<i32>()?;
                            }
                            Rule::clauses => {
                                clauses = def_rule
                                    .as_str()
                                    .parse::<i32>()
                                    .map(|o| o.try_into().unwrap())?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }
    }
    Ok(())
}

/// Reads a DIMACS CNF file from a given path or standard input and parses it into a `CnfFormula`.
pub fn read_dimacs_from_file<P: AsRef<Path>,D:AsDimacs>(
    path: Option<&P>,
    strict: bool,
    dim:&mut D
) -> Result<(), ParserError> {
    let data = match path {
        Some(p) => {
            fs::read_to_string(p)?
        }
        None => {
            let mut buf = String::new();
            let _ = io::stdin().read_to_string(&mut buf);
            buf
        }
    };
    parse_dimacs_cnf(&data, strict,dim)
}
