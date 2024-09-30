#[cfg(feature = "dimacs")]
mod dimacs;
#[cfg(feature = "dimacs")]
pub  use dimacs::parse_dimacs_cnf;
#[cfg(feature = "dimacs")]
pub  use dimacs::read_dimacs_from_file;
#[cfg(feature = "dimacs")]
pub(crate) use dimacs::Rule;

#[cfg(feature = "parser")]
pub struct CnfFormula{
    pub clauses :Vec<Vec<i32>>,
    pub num_vars: usize,
    pub num_clauses: usize, 
}
#[cfg(feature = "parser")]
impl CnfFormula {
    fn new() -> Self {
        Self { clauses:vec![], num_vars: 0, num_clauses: 0 }
    }
}