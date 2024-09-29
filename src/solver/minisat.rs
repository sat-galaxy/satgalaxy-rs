//! The `minisat` module provides access to the `MinisatSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `MinisatSolver` struct acts as a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `minisat` module, ensure the `minisat` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["minisat"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/minisat_bindings.rs"));


use super::base::{Solver, Status};

/// `MinisatSolver` is a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver.
/// It also allows creating a `Minisat_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{MinisatSolver, Status,Solver};
/// let solver = MinisatSolver::new();
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
///  To use the `MinisatSolver`, ensure the `minisat` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["minisat"] }

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
