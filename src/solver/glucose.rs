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
//! satgalaxy = { version = "x.y.z", features = ["glucose"] }
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
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use satgalaxy::solver::{GlucoseSolver, Status,Solver};
/// let solver = GlucoseSolver::new();
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
///  To use the `GlucoseSolver`, ensure the `glucose` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  satgalaxy = { version = "x.y.z", features = ["glucose"] }
pub struct GlucoseSolver(*const c_void);
unsafe impl Sync for GlucoseSolver {}
unsafe impl Send for GlucoseSolver {}
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
            GlucoseSolver(bindings::glucose_new_solver())
        }
    }

    pub fn vars(& self) -> i32 {
        unsafe { bindings::glucose_nvars(self.0) }
    }
    pub fn new_var(& self) -> i32 {
        unsafe { bindings::glucose_new_var(self.0) as i32 }
    }

    pub fn add_clause(& self, clause: &[i32]) {
        unsafe {
            bindings::glucose_add_clause(
                self.0,
                clause.as_ptr(),
                clause.len().try_into().unwrap(),
            );
        }
    }
    pub fn add_empty_clause(& self) {
        unsafe {
            bindings::glucose_add_empty_clause(self.0);
        }
    }
    pub fn value(& self, var: i32) -> bool {
        unsafe { bindings::glucose_value(self.0, var as c_int) != 0 }
    }
    pub fn model_value(& self, var: i32) -> bool {
        unsafe { bindings::glucose_model_value(self.0, var as c_int) != 0 }
    }
    pub fn solve_assumps(& self, assumps: &[i32], do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            bindings::glucose_solve_assumps(
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
            match bindings::glucose_solve_limited(
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
        unsafe { bindings::glucose_solve(self.0, do_simp.into(), turn_off_simp.into()) == 1 }
    }
    pub fn eliminate(& self, turn_off_simp: bool) {
        unsafe {
            bindings::glucose_eliminate(self.0, turn_off_simp.into());
        }
    }
    pub fn assigns(& self) -> usize {
        unsafe { bindings::glucose_nassigns(self.0) as usize }
    }
    pub fn clauses(& self) -> usize {
        unsafe { bindings::glucose_nclauses(self.0) as usize }
    }
    pub fn learnts(& self) -> usize {
        unsafe { bindings::glucose_nlearnts(self.0) as usize }
    }

    pub fn okay(& self) -> bool {
        unsafe { bindings::glucose_okay(self.0) == 1 }
    }

    pub fn model(& self) -> Vec<i32> {
        (1..self.vars() + 1)
            .filter(|lit| self.model_value(*lit))
            .collect()
    }
}

impl SatSolver for GlucoseSolver {
    fn solve_model(& self) -> Status {
        self.eliminate(true);
        match self.solve_limited(&[], true, false) {
            RawStatus::Satisfiable => Status::Satisfiable(self.model()),
            RawStatus::Unsatisfiable => Status::Unsatisfiable,
            RawStatus::Unknown => Status::Unknown,
        }
    }

    fn add_clause(& self, clause: &[i32]) {
        GlucoseSolver::add_clause(self, clause);
    }
}
impl Drop for GlucoseSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::glucose_destroy(self.0);
        }
    }
}
