#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/minisat_bindings.rs"));


use super::base::{Solver, Status};


pub struct MinisatSolver {
    inner: Minisat_StdSimpSolver,
}

impl MinisatSolver {
    pub fn new() -> Self {
        unsafe {
            MinisatSolver {
                inner: Minisat_StdSimpSolver::new(),
            }
        }
    }
    pub fn model(&mut self) -> Vec<i32> {
        let mut m = Vec::<i32>::new();
        unsafe {
            for i in 0..self.inner.nVars(){
                if self.inner.value(i) {
                    m.push(i+1);
                }
            }
        }
        m
    }
}

impl Solver for MinisatSolver {
    fn solve(&mut self) -> Status {
        unsafe {
            self.inner.eliminate(true);
            return if self.inner.solve1(true, false) {
                Status::SATISFIABLE(self.model())
            } else {
                Status::UNSATISFIABLE
            };
        }
    }

    fn add_clause(&mut self, clause: &Vec<i32>) {
        unsafe {
            println!("{}",self.inner.nVars());
            self.inner.addClause(clause.as_ptr(),clause.len());
        }
    }
}
impl Drop for MinisatSolver {
    fn drop(&mut self) {
        unsafe {
            self.inner.destruct();
        }
    }
}
