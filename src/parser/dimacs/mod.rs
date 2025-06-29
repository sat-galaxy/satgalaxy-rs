use std::{
    cmp::max,
    fs::File,
    io::{self, BufReader, Read, Seek, SeekFrom, Stdin},
    path::Path,
};
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;
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
    let mut reader = match path {
        Some(p) => {
            SmartReader::open(p)?
        }
        None => {
            io::stdin().into()
        }
    };
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    parse_dimacs_cnf(&buf, strict,dim)
}
enum SmartReader {
    Plain(BufReader<File>),
    Gzip(BufReader<GzDecoder<File>>),
    Xz(BufReader<XzDecoder<File>>),
    Stdio(BufReader<io::Stdin>),
}

impl SmartReader {
    /// 打开文件并自动检测格式
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        
        let mut header = [0u8; 6];
        let mut temp_reader = BufReader::new(&file);
        temp_reader.read_exact(&mut header)?;
        
        file.seek(SeekFrom::Start(0))?;

        // Gzip file header: 0x1F 0x8B
        match header {
            [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00] => {
                let decoder = XzDecoder::new(file);
                Ok(Self::Xz(BufReader::new(decoder)))
            },
            [0x1F, 0x8B, ..] => {
                let decoder = GzDecoder::new(file);
                Ok(Self::Gzip(BufReader::new(decoder)))
            }
            _=>{
                Ok(Self::Plain(BufReader::new(file)))
            }
        }
        
    }
}

impl Read for SmartReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            SmartReader::Plain(r) => r.read(buf),
            SmartReader::Gzip(r) => r.read(buf),
            SmartReader::Stdio(r) => r.read(buf),
            SmartReader::Xz(r)=>r.read(buf)
        }
    }

}

impl From<Stdin> for SmartReader {
    fn from(stdin: Stdin) -> Self {
        SmartReader::Stdio(BufReader::new(stdin))
    }
}