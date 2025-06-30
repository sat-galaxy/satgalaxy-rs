use crate::{errors::ParserError, parser::AsDimacs};
#[cfg(feature = "compression")]
use flate2::read::GzDecoder;
#[cfg(feature = "compression")]
use std::io::Cursor;
use std::{
    cmp::max,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};
#[cfg(feature = "compression")]
use xz2::read::XzDecoder;

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
pub fn parse_dimacs_cnf<D: AsDimacs>(
    input: &str,
    strict: bool,
    dim: &mut D,
) -> Result<(), ParserError> {
    let mut num_vars = 0;
    let mut variables = 0;
    let mut clauses = 0;
    let mut num_clauses = 0;
    let pairs = DIMACSParser::parse(Rule::file, input)?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::cluase => {
                    if strict {
                        if clauses > 0 && num_clauses >= clauses {
                            return Err(ParserError::TooManyClauses(num_clauses, clauses));
                        }
                        if num_vars > 0 && num_vars >= variables {
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
                    num_clauses += 1;
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
pub fn read_dimacs_from_file<P: AsRef<Path>, D: AsDimacs>(
    path: P,
    strict: bool,
    dim: &mut D,
) -> Result<(), ParserError> {
    let mut reader = File::open(path)?;
    read_dimacs_from_reader(&mut reader, strict, dim)
}

pub fn read_dimacs_from_reader<R: Read, D: AsDimacs>(
    reader: R,
    strict: bool,
    dim: &mut D,
) -> Result<(), ParserError> {
    let mut reader = SmartReader::new(reader)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    parse_dimacs_cnf(&buf, strict, dim)
}

enum SmartReader<R: Read> {
    Plain(BufReader<R>),
    #[cfg(feature = "compression")]
    Gzip(GzDecoder<BufReader<R>>),
    #[cfg(feature = "compression")]
    Xz(XzDecoder<BufReader<R>>),
}

impl<R: Read> Read for SmartReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            SmartReader::Plain(r) => r.read(buf),
            #[cfg(feature = "compression")]
            SmartReader::Gzip(r) => r.read(buf),
            #[cfg(feature = "compression")]
            SmartReader::Xz(r) => r.read(buf),
        }
    }
}
#[cfg(feature = "compression")]
impl<R: Read> SmartReader<io::Chain<Cursor<Vec<u8>>, R>> {
    pub fn new(reader: R) -> Result<Self, io::Error> {
        let mut reader = reader;
        let mut header = [0u8; 6];

        reader.read_exact(&mut header)?;

        let header_cursor = Cursor::new(header[..6].to_vec());
        let chained_reader = BufReader::new(header_cursor.chain(reader));

        // Gzip file header: 0x1F 0x8B
        match header {
            [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00] => {
                let decoder = XzDecoder::new(chained_reader);
                Ok(Self::Xz(decoder))
            }
            [0x1F, 0x8B, ..] => {
                let decoder = GzDecoder::new(chained_reader);
                Ok(Self::Gzip(decoder))
            }
            _ => Ok(Self::Plain(chained_reader)),
        }
    }
}
#[cfg(not(feature = "compression"))]
impl<R: Read> SmartReader<R> {
    pub fn new(reader: R) -> Result<Self, io::Error> {
        Ok(SmartReader::Plain(BufReader::new(reader)))
    }
}
