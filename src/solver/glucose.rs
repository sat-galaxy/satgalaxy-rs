//! The `glucose` module provides access to the `GlucoseSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `GlucoseSolver` struct acts as a wrapper for the [Glucose](https://github.com/audemard/glucose) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `glucose` module, ensure the `glucose` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! rssat = { version = "x.y.z", features = ["glucose"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/glucose_bindings.rs"));

use super::base::{Solver, Status};

/// `GlucoseSolver` is a wrapper for the [Glucose](https://github.com/audemard/glucose) SimpSolver.
/// It also allows creating a `Glucose_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{GlucoseSolver, Status,Solver};
/// let mut solver = GlucoseSolver::new();
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
///  To use the `GlucoseSolver`, ensure the `glucose` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  rssat = { version = "x.y.z", features = ["glucose"] }
pub struct GlucoseSolver {
    inner: Glucose_StdSimpSolver,
}

impl GlucoseSolver {
    pub fn new() -> Self {
        unsafe {
            GlucoseSolver {
                inner: Glucose_StdSimpSolver::new(),
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

impl Solver for GlucoseSolver {
    fn solve_model(&mut self) -> Status {
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
impl Drop for GlucoseSolver {
    fn drop(&mut self) {
        unsafe {
            self.inner.destruct();
        }
    }
}
