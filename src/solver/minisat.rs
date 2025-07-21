//! The `minisat` module provides access to the `MinisatSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `MinisatSolver` struct acts as a wrapper for the [MiniSat](https://github.com/niklasso/minisat), allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `minisat` module, ensure the `minisat` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! satgalaxy = { version = "x.y.z", features = ["minisat"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/minisat_bindings.rs"));
}
use crate::errors::SolverError;

use super::{RawStatus, SatSolver, SatStatus};
use std::ffi::{c_int, c_void};

/// `MinisatSolver` is a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver.
/// It also allows creating a `Minisat_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use satgalaxy::solver::{MinisatSolver, Status,Solver};
/// let solver = MinisatSolver::new();
///     solver.add_clause(&vec![1, 2]);
///     solver.add_clause(&vec![-1, -2]);
///     solver.add_clause(&vec![3]);
///
/// match solver.solve_model() {
///    Status::Satisfiable(vec) => {
///         println!("Satisfiable solution: {:?}", vec);
///     },
///     Status::Unsatisfiable => {
///         println!("Unsatisfiable");
///     },
///     Status::Unknown => {
///         println!("Unknown");
///     },
/// }
/// ```
///  # Usage
///  To use the `MinisatSolver`, ensure the `minisat` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  satgalaxy = { version = "x.y.z", features = ["minisat"] }
///
pub struct MinisatSolver(*mut bindings::MiniSATSolver);
unsafe impl Sync for MinisatSolver {}
unsafe impl Send for MinisatSolver {}

impl Default for MinisatSolver {
    fn default() -> Self {
        Self::new()
    }
}
macro_rules! minisat_opt_set {
    ($name:ident,$type:ty,$doc:expr) => {
        minisat_opt_set!($name, $name, $type, $doc);
    };
    ($name:ident,$ffi_name:ident,$type:ty,$doc:expr) => {
        paste::paste! {
            #[doc=$doc]
            pub fn [<set_opt_$name>](value: $type) -> Result<(), SolverError> {
                let code = unsafe {
                     bindings::[<minisat_set_opt_$ffi_name>](value.into())
                    };

                if code!=0{
                    return Err(SolverError(Self::error_msg(code)));
                }
                Ok(())
            }
        }
    };
}

impl MinisatSolver {
    fn error_msg(code: i32) -> &'static str {
        unsafe {
            let msg = bindings::minisat_error_msg(code);
            let msg = std::ffi::CStr::from_ptr(msg);
            msg.to_str().unwrap()
        }
    }
    minisat_opt_set!(
        var_decay,
        var_decay,
        f64,
        "The variable activity decay factor. \n\n value must be in (0, 1)"
    );
    minisat_opt_set!(
        clause_decay,
        clause_decay,
        f64,
        "The clause activity decay factor. \n\n value must be in (0, 1)"
    );
    minisat_opt_set!(random_var_freq, random_var_freq, f64, "The frequency with which the decision heuristic tries to choose a random variable. \n\n value must be in [0,1]");
    minisat_opt_set!(
        random_seed,
        random_seed,
        f64,
        "Used by the random variable selection. \n\n value must be positive"
    );
    minisat_opt_set!(ccmin_mode, ccmin_mode, i32, "Controls conflict clause minimization. \n\n value must be 0, 1, or 2 (0=none, 1=basic, 2=deep)");
    minisat_opt_set!(phase_saving, phase_saving, i32, "Controls the level of phase saving. \n\n value must be 0, 1, or 2 (0=none, 1=limited, 2=full)");
    minisat_opt_set!(
        rnd_init_act,
        rnd_init_act,
        bool,
        "Randomize the initial activity. "
    );
    minisat_opt_set!(
        luby_restart,
        luby_restart,
        bool,
        "Use the Luby restart sequence. "
    );
    minisat_opt_set!(
        restart_first,
        restart_first,
        i32,
        "The base restart interval. \n\n value must be a positive integer"
    );
    minisat_opt_set!(
        restart_inc,
        restart_inc,
        f64,
        "Restart interval increase factor. \n\n value must be at least 1.0"
    );
    minisat_opt_set!(garbage_frac, garbage_frac, f64, "The fraction of wasted memory allowed before a garbage collection is triggered. \n\n value must be positive");
    minisat_opt_set!(
        min_learnts_lim,
        min_learnts_lim,
        i32,
        "Minimum learnt clause limit. \n\n value must be at least 0"
    );
    minisat_opt_set!(
        use_asymm,
        use_asymm,
        bool,
        "Shrink clauses by asymmetric branching. "
    );
    minisat_opt_set!(
        use_rcheck,
        use_rcheck,
        bool,
        "Check if a clause is already implied (costly). "
    );
    minisat_opt_set!(use_elim, use_elim, bool, "Perform variable elimination. ");
    minisat_opt_set!(grow, grow, i32, "Allow a variable elimination step to grow by a number of clauses. \n\n value must be at least 0");
    minisat_opt_set!(clause_lim, clause_lim, i32, "Variables are not eliminated if it produces a resolvent with a length above this limit. \n\n value must be at least -1 (-1 means no limit)");
    minisat_opt_set!(subsumption_lim, subsumption_lim, i32, "Do not check if subsumption against a clause larger than this. \n\n value must be at least -1 (-1 means no limit)");
    minisat_opt_set!(simp_garbage_frac, simp_garbage_frac, f64, "The fraction of wasted memory allowed before a garbage collection is triggered during simplification. \n\n value must be positive");
    minisat_opt_set!(
        verbosity,
        verbosity,
        i32,
        "Verbosity level. \n\n value must be 0, 1, or 2 (0=silent, 1=some, 2=more)"
    );

    /// create a new solver
    pub fn new() -> Self {
        unsafe { MinisatSolver(bindings::minisat_new_solver()) }
    }
    /// The current number of variables.
    pub fn vars(&mut self) -> i32 {
        unsafe { bindings::minisat_nvars(self.0) }
    }
    /// Create a new variable
    pub fn new_var(&mut self) -> i32 {
        unsafe { bindings::minisat_new_var(self.0) as i32 }
    }
    /// Release a variable.
    pub fn release_var(&mut self, var: i32) {
        unsafe {
            bindings::minisat_release_var(self.0, var as c_int);
        }
    }
    /// Add a clause to the solver.
    pub fn add_clause(&mut self, clause: &[i32]) {
        unsafe {
            bindings::minisat_add_clause(self.0, clause.as_ptr(), clause.len().try_into().unwrap());
        }
    }
    /// Add an empty clause to the solver. (unsat)
    pub fn add_empty_clause(&mut self) {
        unsafe {
            bindings::minisat_add_empty_clause(self.0);
        }
    }
    ///  The current assignments for the variables
    pub fn value(&mut self, var: i32) -> bool {
        unsafe { bindings::minisat_value(self.0, var as c_int) != 0 }
    }
    // The model assignments for the variables
    pub fn model_value(&mut self, var: i32) -> bool {
        unsafe { bindings::minisat_model_value(self.0, var as c_int) != 0 }
    }
    // Solving with assumptions, do_simp (recommend true) and turn_off_simp (recommend false)
    pub fn solve_assumps(&mut self, assumps: &[i32], do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            bindings::minisat_solve_assumps(
                self.0,
                assumps.as_ptr(),
                assumps.len().try_into().unwrap(),
                do_simp.into(),
                turn_off_simp.into(),
            ) == 1
        }
    }
    /// Solving, do_simp (recommend true) and turn_off_simp (recommend false)
    pub fn solve_limited(
        &mut self,
        assumps: &[i32],
        do_simp: bool,
        turn_off_simp: bool,
    ) -> RawStatus {
        unsafe {
            match bindings::minisat_solve_limited(
                self.0,
                assumps.as_ptr(),
                assumps.len().try_into().unwrap(),
                do_simp.into(),
                turn_off_simp.into(),
            ) {
                10 => RawStatus::Satisfiable,
                20 => RawStatus::Unsatisfiable,
                _ => RawStatus::Unknown,
            }
        }
    }
    /// Solving, do_simp (recommend true) and turn_off_simp (recommend false)
    pub fn solve(&mut self, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe { bindings::minisat_solve(self.0, do_simp.into(), turn_off_simp.into()) == 1 }
    }
    /// Perform variable elimination based simplification. turn_off_simp (recommend false)
    pub fn eliminate(&mut self, turn_off_simp: bool) {
        unsafe {
            bindings::minisat_eliminate(self.0, turn_off_simp.into());
        }
    }
    /// The current number of assigned literals.
    pub fn assigns(&mut self) -> usize {
        unsafe { bindings::minisat_nassigns(self.0) as usize }
    }
    /// The current number of original clauses.
    pub fn clauses(&mut self) -> usize {
        unsafe { bindings::minisat_nclauses(self.0) as usize }
    }
    /// The current number of learnt clauses.
    pub fn learnts(&mut self) -> usize {
        unsafe { bindings::minisat_nlearnts(self.0) as usize }
    }

    pub fn okay(&mut self) -> bool {
        unsafe { bindings::minisat_okay(self.0) == 1 }
    }
    /// Get current model if the solver is satisfiable.
    pub fn model(&mut self) -> Vec<i32> {
        (1..self.vars() + 1)
            .filter(|lit| self.model_value(*lit))
            .collect()
    }
}

impl SatSolver for MinisatSolver {
    fn add_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        MinisatSolver::add_clause(self, clause);
        Ok(())
    }

    fn solve(&mut self) -> Result<RawStatus, SolverError> {
        self.eliminate(true);
        Ok(self.solve_limited(&[], true, false))
    }

    fn model(&mut self) -> Result<Vec<i32>, SolverError> {
        Ok(MinisatSolver::model(self))
    }
}
impl Drop for MinisatSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::minisat_destroy(self.0);
        }
    }
}
