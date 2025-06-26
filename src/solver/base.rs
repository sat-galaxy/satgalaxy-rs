


pub enum Status {
    SATISFIABLE(Vec<i32>),UNSATISFIABLE,UNKNOWN
}

impl Default for Status {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

pub trait Solver {

    fn add_clause(& mut self, clause:&Vec<i32>);
    fn solve_model(& mut self)->Status;
}