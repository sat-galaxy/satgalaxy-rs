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

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/glucose_bindings.rs"));
}
use std::{ffi::c_int, os::raw::c_void};

use crate::solver::{RawStatus, SatSolver, Status};

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
    inner: *mut c_void,
}

impl Default for GlucoseSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl GlucoseSolver {
    pub fn set_opt_k(value: f64) {
        unsafe {
            bindings::glucose_set_opt_K(value);
        }
    }

    pub fn set_opt_r(value: f64) {
        unsafe {
            bindings::glucose_set_opt_R(value);
        }
    }

    pub fn set_opt_size_lbd_queue(value: i32) {
        unsafe {
            bindings::glucose_set_opt_size_lbd_queue(value);
        }
    }

    pub fn set_opt_size_trail_queue(value: i32) {
        unsafe {
            bindings::glucose_set_opt_size_trail_queue(value);
        }
    }

    pub fn set_opt_first_reduce_db(value: i32) {
        unsafe {
            bindings::glucose_set_opt_first_reduce_db(value);
        }
    }

    pub fn set_opt_inc_reduce_db(value: i32) {
        unsafe {
            bindings::glucose_set_opt_inc_reduce_db(value);
        }
    }

    pub fn set_opt_spec_inc_reduce_db(value: i32) {
        unsafe {
            bindings::glucose_set_opt_spec_inc_reduce_db(value);
        }
    }

    pub fn set_opt_lb_lbd_frozen_clause(value: i32) {
        unsafe {
            bindings::glucose_set_opt_lb_lbd_frozen_clause(value);
        }
    }

    pub fn set_opt_chanseok_hack(value: bool) {
        unsafe {
            bindings::glucose_set_opt_chanseok_hack(value.into());
        }
    }

    pub fn set_opt_chanseok_limit(value: i32) {
        unsafe {
            bindings::glucose_set_opt_chanseok_limit(value);
        }
    }

    pub fn set_opt_lb_size_minimzing_clause(value: i32) {
        unsafe {
            bindings::glucose_set_opt_lb_size_minimzing_clause(value);
        }
    }

    pub fn set_opt_lb_lbd_minimzing_clause(value: i32) {
        unsafe {
            bindings::glucose_set_opt_lb_lbd_minimzing_clause(value);
        }
    }

    pub fn set_opt_lcm(value: bool) {
        unsafe {
            bindings::glucose_set_opt_lcm(value.into());
        }
    }

    pub fn set_opt_lcm_update_lbd(value: bool) {
        unsafe {
            bindings::glucose_set_opt_lcm_update_lbd(value.into());
        }
    }

    pub fn set_opt_var_decay(value: f64) {
        unsafe {
            bindings::glucose_set_opt_var_decay(value);
        }
    }

    pub fn set_opt_max_var_decay(value: f64) {
        unsafe {
            bindings::glucose_set_opt_max_var_decay(value);
        }
    }

    pub fn set_opt_clause_decay(value: f64) {
        unsafe {
            bindings::glucose_set_opt_clause_decay(value);
        }
    }

    pub fn set_opt_random_var_freq(value: f64) {
        unsafe {
            bindings::glucose_set_opt_random_var_freq(value);
        }
    }

    pub fn set_opt_random_seed(value: f64) {
        unsafe {
            bindings::glucose_set_opt_random_seed(value);
        }
    }

    pub fn set_opt_ccmin_mode(value: i32) {
        unsafe {
            bindings::glucose_set_opt_ccmin_mode(value);
        }
    }

    pub fn set_opt_phase_saving(value: i32) {
        unsafe {
            bindings::glucose_set_opt_phase_saving(value);
        }
    }

    pub fn set_opt_rnd_init_act(value: bool) {
        unsafe {
            bindings::glucose_set_opt_rnd_init_act(value.into());
        }
    }

    pub fn set_opt_garbage_frac(value: f64) {
        unsafe {
            bindings::glucose_set_opt_garbage_frac(value);
        }
    }

    pub fn set_opt_glu_reduction(value: bool) {
        unsafe {
            bindings::glucose_set_opt_glu_reduction(value.into());
        }
    }

    pub fn set_opt_luby_restart(value: bool) {
        unsafe {
            bindings::glucose_set_opt_luby_restart(value.into());
        }
    }

    pub fn set_opt_restart_inc(value: f64) {
        unsafe {
            bindings::glucose_set_opt_restart_inc(value);
        }
    }

    pub fn set_opt_luby_restart_factor(value: i32) {
        unsafe {
            bindings::glucose_set_opt_luby_restart_factor(value);
        }
    }

    pub fn set_opt_randomize_phase_on_restarts(value: i32) {
        unsafe {
            bindings::glucose_set_opt_randomize_phase_on_restarts(value);
        }
    }

    pub fn set_opt_fixed_randomize_phase_on_restarts(value: bool) {
        unsafe {
            bindings::glucose_set_opt_fixed_randomize_phase_on_restarts(value.into());
        }
    }

    pub fn set_opt_adapt(value: bool) {
        unsafe {
            bindings::glucose_set_opt_adapt(value.into());
        }
    }

    pub fn set_opt_forceunsat(value: bool) {
        unsafe {
            bindings::glucose_set_opt_forceunsat(value.into());
        }
    }

    pub fn set_opt_use_asymm(value: bool) {
        unsafe {
            bindings::glucose_set_opt_use_asymm(value.into());
        }
    }

    pub fn set_opt_use_rcheck(value: bool) {
        unsafe {
            bindings::glucose_set_opt_use_rcheck(value.into());
        }
    }

    pub fn set_opt_use_elim(value: bool) {
        unsafe {
            bindings::glucose_set_opt_use_elim(value.into());
        }
    }

    pub fn set_opt_grow(value: i32) {
        unsafe {
            bindings::glucose_set_opt_grow(value);
        }
    }

    pub fn set_opt_clause_lim(value: i32) {
        unsafe {
            bindings::glucose_set_opt_clause_lim(value);
        }
    }

    pub fn set_opt_subsumption_lim(value: i32) {
        unsafe {
            bindings::glucose_set_opt_subsumption_lim(value);
        }
    }

    pub fn set_opt_simp_garbage_frac(value: f64) {
        unsafe {
            bindings::glucose_set_opt_simp_garbage_frac(value);
        }
    }

    pub fn set_opt_verbosity(value: i32) {
        unsafe {
            bindings::glucose_set_opt_verbosity(value);
        }
    }

    pub fn new() -> Self {
        unsafe {
            GlucoseSolver {
                inner: bindings::glucose_new_solver(),
            }
        }
    }

    pub fn vars(&mut self) -> i32 {
        unsafe { bindings::glucose_nvars(self.inner) }
    }
    pub fn new_var(&mut self) -> i32 {
        unsafe { bindings::glucose_new_var(self.inner) as i32 }
    }

    pub fn add_clause(&mut self, clause: &[i32]) {
        unsafe {
            bindings::glucose_add_clause(
                self.inner,
                clause.as_ptr(),
                clause.len().try_into().unwrap(),
            );
        }
    }
    pub fn add_empty_clause(&mut self) {
        unsafe {
            bindings::glucose_add_empty_clause(self.inner);
        }
    }
    pub fn value(&mut self, var: i32) -> bool {
        unsafe { bindings::glucose_value(self.inner, var as c_int) != 0 }
    }
    pub fn model_value(&mut self, var: i32) -> bool {
        unsafe { bindings::glucose_model_value(self.inner, var as c_int) != 0 }
    }
    pub fn solve_assumps(&mut self, assumps: &[i32], do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            bindings::glucose_solve_assumps(
                self.inner,
                assumps.as_ptr(),
                assumps.len().try_into().unwrap(),
                do_simp.into(),
                turn_off_simp.into(),
            ) == 1
        }
    }

    pub fn solve_limited(
        &mut self,
        assumps: &[i32],
        do_simp: bool,
        turn_off_simp: bool,
    ) -> RawStatus {
        unsafe {
            match bindings::glucose_solve_limited(
                self.inner,
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

    pub fn solve(&mut self, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe { bindings::glucose_solve(self.inner, do_simp.into(), turn_off_simp.into()) == 1 }
    }
    pub fn eliminate(&mut self, turn_off_simp: bool) {
        unsafe {
            bindings::glucose_eliminate(self.inner, turn_off_simp.into());
        }
    }
    pub fn assigns(&mut self) -> usize {
        unsafe { bindings::glucose_nassigns(self.inner) as usize }
    }
    pub fn clauses(&mut self) -> usize {
        unsafe { bindings::glucose_nclauses(self.inner) as usize }
    }
    pub fn learnts(&mut self) -> usize {
        unsafe { bindings::glucose_nlearnts(self.inner) as usize }
    }

    pub fn okay(&mut self) -> bool {
        unsafe { bindings::glucose_okay(self.inner) == 1 }
    }

    pub fn model(&mut self) -> Vec<i32> {
        (1..self.vars() + 1)
            .filter(|lit| self.model_value(*lit))
            .collect()
    }
}

impl SatSolver for GlucoseSolver {
    fn solve_model(&mut self) -> Status {
        self.eliminate(true);
        match self.solve_limited(&[], true, false) {
            RawStatus::Satisfiable => Status::Satisfiable(self.model()),
            RawStatus::Unsatisfiable => Status::Unsatisfiable,
            RawStatus::Unknown => Status::Unknown,
        }
    }

    fn add_clause(&mut self, clause: &[i32]) {
        GlucoseSolver::add_clause(self, clause);
    }
}
impl Drop for GlucoseSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::glucose_destroy(self.inner);
        }
    }
}
