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
use super::{RawStatus, SatSolver, Status};
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
pub struct MinisatSolver(*const c_void);
unsafe impl Sync for MinisatSolver {}
unsafe impl Send for MinisatSolver {}

impl Default for MinisatSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MinisatSolver {
    /// The variable activity decay factor(0~1). default: 0.95
    pub fn set_opt_var_decay(decay: f64) {
        unsafe {
            bindings::minisat_set_opt_var_decay(decay);
        }
    }
    /// The clause activity decay factor default: 0.999
    pub fn set_opt_clause_decay(decay: f64) {
        unsafe {
            bindings::minisat_set_opt_clause_decay(decay);
        }
    }

    /// The frequency with which the decision heuristic tries to choose a random variable
    pub fn set_opt_random_var_freq(freq: f64) {
        unsafe {
            bindings::minisat_set_opt_random_var_freq(freq);
        }
    }

    pub fn set_opt_random_seed(seed: f64) {
        unsafe {
            bindings::minisat_set_opt_random_seed(seed);
        }
    }

    pub fn set_opt_ccmin_mode(mode: i32) {
        unsafe {
            bindings::minisat_set_opt_ccmin_mode(mode);
        }
    }

    pub fn set_opt_phase_saving(mode: i32) {
        unsafe {
            bindings::minisat_set_opt_phase_saving(mode);
        }
    }

    pub fn set_opt_rnd_init_act(flag: bool) {
        unsafe {
            bindings::minisat_set_opt_rnd_init_act(flag.into());
        }
    }

    pub fn set_opt_luby_restart(flag: bool) {
        unsafe {
            bindings::minisat_set_opt_luby_restart(flag.into());
        }
    }
    pub fn set_opt_restart_first(restart_first: i32) {
        unsafe {
            bindings::minisat_set_opt_restart_first(restart_first);
        }
    }
    pub fn set_opt_restart_inc(restart_inc: f64) {
        unsafe {
            bindings::minisat_set_opt_restart_inc(restart_inc);
        }
    }
    pub fn set_opt_min_learnts_lim(min_learnts_lim: i32) {
        unsafe {
            bindings::minisat_set_opt_min_learnts_lim(min_learnts_lim);
        }
    }
    pub fn set_opt_use_asymm(opt_use_asymm: bool) {
        unsafe {
            bindings::minisat_set_opt_use_asymm(opt_use_asymm.into());
        }
    }

    pub fn set_opt_use_rcheck(opt_use_rcheck: bool) {
        unsafe {
            bindings::minisat_set_opt_use_rcheck(opt_use_rcheck.into());
        }
    }

    pub fn set_opt_use_elim(opt_use_elim: bool) {
        unsafe {
            bindings::minisat_set_opt_use_elim(opt_use_elim.into());
        }
    }

    pub fn set_opt_grow(opt_grow: i32) {
        unsafe {
            bindings::minisat_set_opt_grow(opt_grow);
        }
    }

    pub fn set_opt_clause_lim(opt_clause_lim: i32) {
        unsafe {
            bindings::minisat_set_opt_clause_lim(opt_clause_lim);
        }
    }
    pub fn set_opt_subsumption_lim(opt_subsumption_lim: i32) {
        unsafe {
            bindings::minisat_set_opt_subsumption_lim(opt_subsumption_lim);
        }
    }

    pub fn set_opt_simp_garbage_frac(opt_simp_garbage_frac: f64) {
        unsafe {
            bindings::minisat_set_opt_simp_garbage_frac(opt_simp_garbage_frac);
        }
    }
    pub fn set_opt_garbage_frac(garbage_frac: f64) {
        unsafe {
            bindings::minisat_set_opt_garbage_frac(garbage_frac);
        }
    }
    pub fn set_opt_verbosity(verb: i32) {
        unsafe {
            bindings::minisat_set_opt_verbosity(verb);
        }
    }

    pub fn new() -> Self {
        unsafe {
            MinisatSolver(
                bindings::minisat_new_solver()
            )
        }
    }
    pub fn vars(& self) -> i32 {
        unsafe { bindings::minisat_nvars(self.0) }
    }
    pub fn new_var(& self) -> i32 {
        unsafe { bindings::minisat_new_var(self.0) as i32 }
    }
    pub fn release_var(& self, var: i32) {
        unsafe {
            bindings::minisat_release_var(self.0, var as c_int);
        }
    }

    pub fn add_clause(& self, clause: &[i32]) {
        unsafe {
            bindings::minisat_add_clause(
                self.0,
                clause.as_ptr(),
                clause.len().try_into().unwrap(),
            );
        }
    }
    pub fn add_empty_clause(& self) {
        unsafe {
            bindings::minisat_add_empty_clause(self.0);
        }
    }
    pub fn value(& self, var: i32) -> bool {
        unsafe { bindings::minisat_value(self.0, var as c_int) != 0 }
    }
    pub fn model_value(& self, var: i32) -> bool {
        unsafe { bindings::minisat_model_value(self.0, var as c_int) != 0 }
    }
    pub fn solve_assumps(& self, assumps: &[i32], do_simp: bool, turn_off_simp: bool) -> bool {
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

    pub fn solve_limited(
        & self,
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

    pub fn solve(& self, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            bindings::minisat_solve(self.0, do_simp.into(), turn_off_simp.into()) == 1
        }
    }
    pub fn eliminate(& self, turn_off_simp: bool) {
        unsafe {
            bindings::minisat_eliminate(self.0, turn_off_simp.into());
        }
    }
    pub fn assigns(& self) -> usize {
        unsafe { bindings::minisat_nassigns(self.0) as usize }
    }
    pub fn clauses(& self) -> usize {
        unsafe { bindings::minisat_nclauses(self.0) as usize }
    }
    pub fn learnts(& self) -> usize {
        unsafe { bindings::minisat_nlearnts(self.0) as usize }
    }

    pub fn okay(& self) -> bool {
        unsafe { bindings::minisat_okay(self.0) == 1 }
    }

    pub fn model(& self) -> Vec<i32> {
        (1..self.vars() + 1)
            .filter(|lit| self.model_value(*lit))
            .collect()
    }
}

impl SatSolver for MinisatSolver {
    fn solve_model(& self) -> Status {
        self.eliminate(true);
        match self.solve_limited(&[], true, false) {
            RawStatus::Satisfiable => Status::Satisfiable(self.model()),
            RawStatus::Unsatisfiable => Status::Unsatisfiable,
            RawStatus::Unknown => Status::Unknown,
        }
    }

    fn add_clause(&self, clause: &[i32]) {
        MinisatSolver::add_clause(self, clause);
    }
}
impl Drop for MinisatSolver {
    fn drop(& mut self) {
        unsafe {
            bindings::minisat_destroy(self.0);
        }
    }
}
