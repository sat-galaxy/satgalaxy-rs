


pub enum Status {
    SATISFIABLE(Vec<i32>),UNSATISFIABLE,UNKNOWN
}

pub trait Solver {

     fn add_clause(& mut self, clause:&Vec<i32>);
    fn solve(& mut self)->Status;
}