//! The `cadical` module provides access to the `CaDiCaLSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `CaDiCaLSolver` struct acts as a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `cadical` module, ensure the `cadical` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["cadical"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/cadical_bindings.rs"));


use super::base::{Solver, Status};

/// `CaDiCaLSolver` is a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver .
/// It also allows creating a `CaDiCaL_Solver` instance for more low-level operations.
/// This struct is only available when the `cadical` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{CaDiCaLSolver, Status,Solver};
/// let mut solver = CaDiCaLSolver::new();
///     solver.add_clause(&vec![1, 2]);
///     solver.add_clause(&vec![-1, -2]);
///     solver.add_clause(&vec![3]);
/// 
/// match solver.solve() {
///    Status::SATISFIABLE(vec) => {
///         println!("Satisfiable solution: {:?}", vec);
///     },
///     Status::UNSATISFIABLE => {
///         println!("Unsatisfiable");
///     },
///     Status::UNKNOWN => {
///         println!("Unknown");
///     },
/// }
/// ```
///  # Usage
///  To use the `CaDiCaLSolver`, ensure the `cadical` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["cadical"] }
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
     fn solve_model(&mut self) -> Status {
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