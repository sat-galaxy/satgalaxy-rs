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

mod bindings {
include!(concat!(env!("OUT_DIR"), "/minisat_bindings.rs"));
}
use super::base::{Solver, Status};
use std::ffi::{c_int, c_void};

/// `MinisatSolver` is a wrapper for the [MiniSat](https://github.com/niklasso/minisat) SimpSolver.
/// It also allows creating a `bindings::minisat_StdSimpSolver` instance for more low-level operations.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use rssat::solver::{MinisatSolver, Status,Solver};
/// let mut solver = MinisatSolver::new();
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
    inner: *mut c_void,
}

impl MinisatSolver {
    /// The variable activity decay factor(0~1). default: 0.95
    pub fn set_opt_var_decay(decay: f64){
        unsafe {
            bindings::minisat_set_opt_var_decay(decay);
        }
    }
    /// The clause activity decay factor default: 0.999
    pub fn set_opt_clause_decay(decay: f64){
        unsafe {
            bindings::minisat_set_opt_clause_decay(decay);
        }
    }

    /// The frequency with which the decision heuristic tries to choose a random variable
    pub fn set_opt_random_var_freq(freq: f64){
        unsafe {
            bindings::minisat_set_opt_random_var_freq(freq);
        }
    }

    pub fn set_opt_random_seed(seed: f64){
        unsafe {
            bindings::minisat_set_opt_random_seed(seed);
        }
    }

    pub fn set_opt_ccmin_mode(mode: i32){
        unsafe {
            bindings::minisat_set_opt_ccmin_mode(mode);
        }
    }

    pub fn set_opt_phase_saving(mode: i32){
        unsafe {
            bindings::minisat_set_opt_phase_saving(mode);
        }
    }

    pub fn set_opt_rnd_init_act(flag: i32){
        unsafe {
            bindings::minisat_set_opt_rnd_init_act(flag);
        }
    }

    pub fn set_opt_luby_restart(flag: i32){
        unsafe {
            bindings::minisat_set_opt_luby_restart(flag);
        }
    }
    pub fn set_opt_restart_first(restart_first: i32){
        unsafe {
            bindings::minisat_set_opt_restart_first(restart_first);
        }
    }
    pub fn set_opt_restart_inc(restart_inc: f64){
        unsafe {
            bindings::minisat_set_opt_restart_inc(restart_inc);
        }
    }
    pub fn set_opt_min_learnts_lim(min_learnts_lim: i32){
        unsafe {
            bindings::minisat_set_opt_min_learnts_lim(min_learnts_lim);
        }
    }
    pub fn set_opt_use_asymm(opt_use_asymm: i32){
        unsafe {
            bindings::minisat_set_opt_use_asymm(opt_use_asymm);
        }
    }

    pub fn set_opt_use_rcheck(opt_use_rcheck: i32){
        unsafe {
            bindings::minisat_set_opt_use_rcheck(opt_use_rcheck);
        }
    }

    pub fn set_opt_use_elim(opt_use_elim: i32){
        unsafe {
            bindings::minisat_set_opt_use_elim(opt_use_elim);
        }
    }

    pub fn set_opt_grow(opt_grow: i32){
        unsafe {
            bindings::minisat_set_opt_grow(opt_grow);
        }
    }

    pub fn set_opt_clause_lim(opt_clause_lim: i32){
        unsafe {
            bindings::minisat_set_opt_clause_lim(opt_clause_lim);
        }
    }
    pub fn set_opt_subsumption_lim(opt_subsumption_lim: i32){
        unsafe {
            bindings::minisat_set_opt_subsumption_lim(opt_subsumption_lim);
        }
    }

    pub fn set_opt_simp_garbage_frac(opt_simp_garbage_frac: f64){
        unsafe {
            bindings::minisat_set_opt_simp_garbage_frac(opt_simp_garbage_frac);
        }
    }
    pub fn set_opt_garbage_frac(garbage_frac:f64){
        unsafe {
            bindings::minisat_set_opt_garbage_frac(garbage_frac);
        }
    } 
   
    pub fn new() -> Self {
        unsafe {
            MinisatSolver {
                inner: bindings::minisat_new_solver(),
            }
        }
    }
    pub fn vars(&mut self) -> usize {
        unsafe { bindings::minisat_nvars(self.inner) as usize }
    }
    pub fn new_var(&mut self) -> i32 {
        unsafe { bindings::minisat_new_var(self.inner) as i32 }
    }
    pub fn release_var(&mut self, var: i32) {
        unsafe {
            bindings::minisat_release_var(self.inner, var as c_int);
        }
    }

    pub fn add_clause(&mut self, clause: &Vec<i32>) {
        unsafe {
            bindings::minisat_add_clause(
                self.inner,
                clause.as_ptr(),
                clause.len().try_into().unwrap(),
            );
        }
    }
    pub fn add_empty_clause(&mut self) {
        unsafe {
            bindings::minisat_add_empty_clause(self.inner);
        }
    }
    pub fn value(&mut self, var: i32) -> bool {
        unsafe { bindings::minisat_value(self.inner, var as c_int) != 0 }
    }

    pub fn solve_assumps(&mut self, assumps: &Vec<i32>, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            return bindings::minisat_solve_assumps(
                self.inner,
                assumps.as_ptr(),
                assumps.len().try_into().unwrap(),
                do_simp.into(),
                turn_off_simp.into(),
            ) == 1;
        }
    }

    pub fn solve_limited(&mut self, assumps: &Vec<i32>, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            return bindings::minisat_solve_limited(
                self.inner,
                assumps.as_ptr(),
                assumps.len().try_into().unwrap(),
                do_simp.into(),
                turn_off_simp.into(),
            ) == 1;
        }
    }

    pub fn solve(&mut self, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe {
            return bindings::minisat_solve(self.inner, do_simp.into(), turn_off_simp.into()) == 1;
        }
    }
    pub fn eliminate(&mut self, turn_off_simp: bool) {
        unsafe {
            bindings::minisat_eliminate(self.inner, turn_off_simp.into());
        }
    }
    pub fn assigns(&mut self) -> usize {
        unsafe { bindings::minisat_nassigns(self.inner) as usize }
    }
    pub fn clauses(&mut self) -> usize {
        unsafe { bindings::minisat_nclauses(self.inner) as usize }
    }
    pub fn learnts(&mut self) -> usize {
        unsafe { bindings::minisat_nlearnts(self.inner) as usize }
    }

    pub fn model(&mut self) -> Vec<i32> {
        let mut m = Vec::<i32>::new();
        for i in 0..self.vars() {
            if self.value(i.try_into().unwrap()) {
                m.push((i + 1).try_into().unwrap());
            }
        }
        m
    }
}

impl Solver for MinisatSolver {
    fn solve_model(&mut self) -> Status {
        self.eliminate(true);
        return if self.solve(true, false) {
            Status::SATISFIABLE(self.model())
        } else {
            Status::UNSATISFIABLE
        };
    }

    fn add_clause(&mut self, clause: &Vec<i32>) {
        MinisatSolver::add_clause(self, clause);
    }
}
impl Drop for MinisatSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::minisat_destroy(self.inner);
        }
    }
}
