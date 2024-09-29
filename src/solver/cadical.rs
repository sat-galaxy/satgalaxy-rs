#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::base::{Solver, Status};

include!(concat!(env!("OUT_DIR"), "/cadical_bindings.rs"));

// 你可以在这里为 CaDiCaL::Solver 创建一个更加 Rust 风格的封装
pub struct CaDiCaLSolver {
    inner:  CaDiCaL_Solver,
}

impl CaDiCaLSolver {
    pub fn new() -> Self {
        unsafe {
            CaDiCaLSolver {
                inner:  CaDiCaL_Solver::new(),
            }
        }
    }

    pub fn val(&mut self, lit: i32) -> i32 {
        unsafe {
          self.inner.val(lit)
        }
    }
    pub  fn model(&mut self)->Vec<i32> {
        let mut m =Vec::<i32>::new();
        unsafe {
        for i in 1..self.inner.vars()+1 {
            if self.val(i)>0 {
                m.push(i);
            }
        }
    }
        m
    }
}

impl Solver for CaDiCaLSolver {
     fn solve(&mut self) -> Status {
        unsafe {
           return  match self.inner.solve() {
                10 => {
                    Status::SATISFIABLE(self.model())
                },
                20 =>{
                    Status::UNSATISFIABLE
                },
                _ => Status::UNKNOWN,
            }
        }
    }
     fn add_clause(&mut self, clause: &Vec<i32>) {
        unsafe {
            self.inner.clause6(clause.as_ptr(),clause.len());
        }
    }
}
impl Drop for CaDiCaLSolver {
    fn drop(&mut self) {
        unsafe {
            self.inner.destruct();
        }
    }
}