mod dimacs;
pub use dimacs::parse_dimacs_cnf;
pub use dimacs::read_dimacs_from_file;
pub use dimacs::read_dimacs_from_reader;
pub(crate) use dimacs::Rule;

use crate::errors::ParserError;
use crate::solver::SatSolver;

/// A problem to be solved.
#[cfg(feature = "parser")]
pub struct Problem {
    pub clauses: Vec<Vec<i32>>,
    pub num_vars: usize,
    pub num_clauses: usize,
}
#[cfg(feature = "parser")]
impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

impl Problem {
    pub fn new() -> Self {
        Self {
            clauses: vec![],
            num_vars: 0,
            num_clauses: 0,
        }
    }
}

pub trait AsDimacs {
    /// Adds a clause to the underlying structure.
    fn push_clause(&mut self, clause: Vec<i32>)->Result<(),ParserError>;
    /// Adds a comment line. Implementations can choose to store or ignore comments.
    fn add_comment(&mut self, comment: String);
}

impl<T: SatSolver> AsDimacs for T {
    fn push_clause(&mut self, clause: Vec<i32>) ->Result<(),ParserError>{
        SatSolver::push_clause(self, &clause)?;
        Ok(())
    }
    fn add_comment(&mut self, _comment: String) {}
}

impl AsDimacs for Vec<Vec<i32>> {
    fn push_clause(&mut self, clause: Vec<i32>)->Result<(),ParserError> {
        self.push(clause);
        Ok(())
    }

    fn add_comment(&mut self, _comment: String) {
        // todo!()
    }
}

impl AsDimacs for Problem {
    fn push_clause(&mut self, clause: Vec<i32>) ->Result<(),ParserError> {
        let max = clause.iter().map(|v| v.abs()).max().unwrap_or(0);
        self.num_vars = self.num_vars.max(max as usize);
        self.clauses.push(clause);
        self.num_clauses += 1;
        Ok(())
    }
    fn add_comment(&mut self, _comment: String) {}
}
