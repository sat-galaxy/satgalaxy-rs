mod dimacs;
pub  use dimacs::parse_dimacs_cnf;
pub  use dimacs::read_dimacs_from_file;
pub(crate) use dimacs::Rule;

use crate::solver::SatSolver;

#[cfg(feature = "parser")]
pub struct Problem{
    pub clauses :Vec<Vec<i32>>,
    pub num_vars: usize,
    pub num_clauses: usize, 
}
#[cfg(feature = "parser")]
impl Problem {
    fn new() -> Self {
        Self { clauses:vec![], num_vars: 0, num_clauses: 0 }
    }
}

pub trait AsDimacs {
    fn add_clause(&mut self, clause: Vec<i32>);
    fn add_comment(&mut self,comment:String);
}

impl<T: SatSolver> AsDimacs for T{
    fn add_clause(&mut self, clause: Vec<i32>) {
        self.add_clause(&clause);
    }
    fn add_comment(&mut self, comment: String) {
    }
}

impl AsDimacs for Vec<Vec<i32>> {
    fn add_clause(&mut self, clause: Vec<i32>) {
        self.push(clause);
    }

    fn add_comment(&mut self,comment:String) {
        // todo!()
    }
}


impl AsDimacs for Problem {
    fn add_clause(&mut self, clause: Vec<i32>) {
        let max= clause.iter().map(|v|v.abs()).max().unwrap_or(0);
        self.num_vars = self.num_vars.max(max as usize);
        self.clauses.push(clause);
        self.num_clauses += 1;
    }
    fn add_comment(&mut self,comment:String) {
        
    }
}