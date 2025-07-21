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

use crate::{
    errors::SolverError,
    solver::{RawStatus, SatSolver, SatStatus},
};

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

macro_rules! glucose_opt_set {
    ($name:ident,$type:ty,$doc:expr) => {
        glucose_opt_set!($name, $name, $type, $doc);
    };
    ($name:ident,$ffi_name:ident,$type:ty,$doc:expr) => {
        paste::paste! {
            #[doc=$doc]
            pub fn [<set_opt_$name>](value: $type) -> Result<(), SolverError> {
                let code = unsafe {
                     bindings::[<glucose_set_opt_$ffi_name>](value.into())
                    };

                if code!=0{
                    return Err(SolverError(Self::error_msg(code)));
                }
                Ok(())
            }
        }
    };
}
impl GlucoseSolver {
    fn error_msg(code: i32) -> &'static str {
        unsafe {
            let msg: *const ::std::os::raw::c_char = bindings::glucose_error_msg(code);
            let msg = std::ffi::CStr::from_ptr(msg);
            msg.to_str().unwrap()
        }
    }
    glucose_opt_set!(
        k,
        K,
        f64,
        "The constant used to force restart.\n value must be in (0, 1)"
    );
    glucose_opt_set!(
        r,
        R,
        f64,
        "The constant used to block restart\n\n# Arguments\n* `value` - must be in (0, 5)"
    );
    glucose_opt_set!(size_lbd_queue, i32, "The size of moving average for LBD (restarts)\n\n# Arguments\n* `value` - must be at least 10");
    glucose_opt_set!(size_trail_queue, i32, "The size of moving average for trail (block restarts)\n\n# Arguments\n* `value` - must be at least 10");
    glucose_opt_set!(first_reduce_db, i32, "The number of conflicts before first reduce DB\n\n# Arguments\n* `value` - must be non-negative");
    glucose_opt_set!(
        inc_reduce_db,
        i32,
        "Increment for reduce DB\n\n# Arguments\n* `value` - must be non-negative"
    );
    glucose_opt_set!(
        spec_inc_reduce_db,
        i32,
        "Special increment for reduce DB\n\n# Arguments\n* `value` - must be non-negative"
    );
    glucose_opt_set!(lb_lbd_frozen_clause, i32, "Protect clauses if LBD decreases below this\n\n# Arguments\n* `value` - must be non-negative");
    glucose_opt_set!(chanseok_hack, bool, "Use Chanseok Oh strategy for LBD");
    glucose_opt_set!(
        chanseok_limit,
        i32,
        "Chanseok: permanent clauses with LBD<=limit\n\n# Arguments\n* `value` - must be > 1"
    );
    glucose_opt_set!(
        lb_size_minimzing_clause,
        i32,
        "Min size required to minimize clause\n\n# Arguments\n* `value` - must be >= 3"
    );
    glucose_opt_set!(
        lb_lbd_minimzing_clause,
        i32,
        "Min LBD required to minimize clause\n\n# Arguments\n* `value` - must be >= 3"
    );
    glucose_opt_set!(lcm, bool, "Use inprocessing vivif (ijcai17)");
    glucose_opt_set!(lcm_update_lbd, bool, "Update LBD when doing LCM");
    glucose_opt_set!(
        var_decay,
        f64,
        "Variable activity decay factor\n\n# Arguments\n* `value` - must be in (0, 1)"
    );
    glucose_opt_set!(
        max_var_decay,
        f64,
        "Max variable activity decay factor\n\n# Arguments\n* `value` - must be in (0, 1)"
    );
    glucose_opt_set!(
        clause_decay,
        f64,
        "Clause activity decay factor\n\n# Arguments\n* `value` - must be in (0, 1)"
    );
    glucose_opt_set!(
        random_var_freq,
        f64,
        "Frequency for random variable selection\n\n# Arguments\n* `value` - must be in [0, 1]"
    );
    glucose_opt_set!(
        random_seed,
        f64,
        "Seed for random variable selection\n\n# Arguments\n* `value` - must be positive"
    );
    glucose_opt_set!(ccmin_mode, i32, "Conflict clause minimization (0=none,1=basic,2=deep)\n\n# Arguments\n* `value` - must be 0-2");
    glucose_opt_set!(
        phase_saving,
        i32,
        "Phase saving (0=none,1=basic,2=deep)\n\n# Arguments\n* `value` - must be 0-2"
    );
    glucose_opt_set!(rnd_init_act, bool, "Randomize initial activity\n\n");
    glucose_opt_set!(
        garbage_frac,
        f64,
        "Memory waste allowed before GC\n\n# Arguments\n* `value` - must be positive"
    );
    glucose_opt_set!(glu_reduction, bool, "Glucose reduction strategy");
    glucose_opt_set!(luby_restart, bool, "Use Luby restart sequence");
    glucose_opt_set!(
        restart_inc,
        f64,
        "Restart interval increase factor\n\n# Arguments\n* `value` - must be >= 1.0"
    );
    glucose_opt_set!(
        luby_restart_factor,
        i32,
        "Luby restart factor\n\n# Arguments\n* `value` - must be positive"
    );
    glucose_opt_set!(
        randomize_phase_on_restarts,
        i32,
        "Randomization level on restarts (0-3)\n\n# Arguments\n* `value` - must be 0-3"
    );
    glucose_opt_set!(
        fixed_randomize_phase_on_restarts,
        bool,
        "Fix first 7 levels at random phase"
    );
    glucose_opt_set!(adapt, bool, "Adapt strategies after 100000 conflicts\n\n# Arguments\n* `value` - boolean (1=true, 0=false)");
    glucose_opt_set!(forceunsat, bool, "Force phase for UNSAT");
    glucose_opt_set!(use_asymm, bool, "Shrink clauses by asymmetric branching");
    glucose_opt_set!(use_rcheck, bool, "Check if clause is already implied");
    glucose_opt_set!(use_elim, bool, "Perform variable elimination");
    glucose_opt_set!(
        grow,
        i32,
        "Allow clause growth in elimination\n\n# Arguments\n* `value` - must be an integer"
    );
    glucose_opt_set!(clause_lim, i32, "Max resolvent length for elimination (-1=no limit)\n\n# Arguments\n* `value` - must be -1 or positive");
    glucose_opt_set!(subsumption_lim, i32, "Max clause size for subsumption (-1=no limit)\n\n# Arguments\n* `value` - must be -1 or positive");
    glucose_opt_set!(
        simp_garbage_frac,
        f64,
        "Memory waste allowed during simplification\n\n# Arguments\n* `value` - must be positive"
    );
    glucose_opt_set!(
        verbosity,
        i32,
        "Verbosity level (0=silent,1=some,2=more)\n\n# Arguments\n* `value` - must be 0-2"
    );

    pub fn new() -> Self {
        unsafe { GlucoseSolver(bindings::glucose_new_solver()) }
    }

    pub fn vars(&mut self) -> i32 {
        unsafe { bindings::glucose_nvars(self.0) }
    }
    pub fn new_var(&mut self) -> i32 {
        unsafe { bindings::glucose_new_var(self.0) as i32 }
    }

    pub fn add_clause(&mut self, clause: &[i32]) {
        unsafe {
            bindings::glucose_add_clause(self.0, clause.as_ptr(), clause.len().try_into().unwrap());
        }
    }
    pub fn add_empty_clause(&mut self) {
        unsafe {
            bindings::glucose_add_empty_clause(self.0);
        }
    }
    pub fn value(&mut self, var: i32) -> bool {
        unsafe { bindings::glucose_value(self.0, var as c_int) != 0 }
    }
    pub fn model_value(&mut self, var: i32) -> bool {
        unsafe { bindings::glucose_model_value(self.0, var as c_int) != 0 }
    }
    pub fn solve_assumps(&mut self, assumps: &[i32], do_simp: bool, turn_off_simp: bool) -> bool {
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
        &mut self,
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

    pub fn solve(&mut self, do_simp: bool, turn_off_simp: bool) -> bool {
        unsafe { bindings::glucose_solve(self.0, do_simp.into(), turn_off_simp.into()) == 1 }
    }
    pub fn eliminate(&mut self, turn_off_simp: bool) {
        unsafe {
            bindings::glucose_eliminate(self.0, turn_off_simp.into());
        }
    }
    pub fn assigns(&mut self) -> usize {
        unsafe { bindings::glucose_nassigns(self.0) as usize }
    }
    pub fn clauses(&mut self) -> usize {
        unsafe { bindings::glucose_nclauses(self.0) as usize }
    }
    pub fn learnts(&mut self) -> usize {
        unsafe { bindings::glucose_nlearnts(self.0) as usize }
    }

    pub fn okay(&mut self) -> bool {
        unsafe { bindings::glucose_okay(self.0) == 1 }
    }

    pub fn model(&mut self) -> Vec<i32> {
        (1..self.vars() + 1)
            .filter(|lit| self.model_value(*lit))
            .collect()
    }
}

impl SatSolver for GlucoseSolver {
    fn add_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        GlucoseSolver::add_clause(self, clause);
        Ok(())
    }
    fn solve(&mut self) -> Result<RawStatus, SolverError> {
        self.eliminate(true);
        Ok(self.solve_limited(&[], true, false))
    }

    fn model(&mut self) -> Result<Vec<i32>, SolverError> {
        Ok(GlucoseSolver::model(self))
    }
}
impl Drop for GlucoseSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::glucose_destroy(self.0);
        }
    }
}
