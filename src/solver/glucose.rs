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
    include!("../../bindings/glucose_bindings.rs");
}
use std::{ffi::c_int, ptr::NonNull};

use crate::{
    errors::SolverError,
    solver::{RawStatus, SatSolver},
};

/// `GlucoseSolver` is a wrapper for the [Glucose](https://github.com/audemard/glucose) SimpSolver.
/// This struct is only available when the `minisat` feature is enabled.
/// # Example
/// ```rust
/// use satgalaxy::solver::{GlucoseSolver, SatStatus, SatSolver};
/// let solver = GlucoseSolver::new();
///     solver.add_clause(&vec![1, 2]);
///     solver.add_clause(&vec![-1, -2]);
///     solver.add_clause(&vec![3]);
///
/// match solver.solve_model() {
///    SatStatus::Satisfiable(vec) => {
///         println!("Satisfiable solution: {:?}", vec);
///     },
///     SatStatus::Unsatisfiable => {
///         println!("Unsatisfiable");
///     },
///     SatStatus::Unknown => {
///         println!("Unknown");
///     },
/// }
/// ```
///  # Usage
///  To use the `GlucoseSolver`, ensure the `glucose` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  satgalaxy = { version = "x.y.z", features = ["glucose"] }
#[derive(Debug, Clone)]
pub struct GlucoseSolver {
    inner: NonNull<bindings::GlucoseSolver>,
}
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
            pub fn [<set_global_opt_$name>](value: $type) -> Result<(), SolverError> {
                let code = unsafe {
                     bindings::[<glucose_set_global_opt_$ffi_name>](value.into())
                    };

                if code!=0{
                    GlucoseSolver::error_msg(code)?;
                }
                Ok(())
            }

            #[doc=$doc]
            pub fn [<set_opt_$name>](&mut self, value: $type) -> Result<(), SolverError> {
                let code = unsafe {
                     bindings::[<glucose_set_opt_$ffi_name>](self.inner.as_ptr(),value.into())
                    };

                if code!=0{
                    GlucoseSolver::error_msg(code)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! glucose_opt_g_set {
    ($name:ident,$type:ty,$doc:expr) => {
        glucose_opt_g_set!($name, $name, $type, $doc);
    };
    ($name:ident,$ffi_name:ident,$type:ty,$doc:expr) => {
        paste::paste! {
            #[doc=$doc]
            pub fn [<set_global_opt_$name>](value: $type) -> Result<(), SolverError> {
                let code = unsafe {
                     bindings::[<glucose_set_global_opt_$ffi_name>](value.into())
                    };

                if code!=0{
                    GlucoseSolver::error_msg(code)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! ffi_bind {
    (
        $(#[$doc:meta])*
        $c_name:ident ($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        as $rust_name:ident
    ) => {
        $(#[$doc])*
        pub fn $rust_name(&mut self, $($arg: $arg_ty),*) -> Result<$ret, SolverError> {
            unsafe {
                let ret = bindings::$c_name(self.inner.as_ptr() $(, $arg.into())*);
                self.error()?;
                Ok(ret.into())
            }
        }
    };
    (
        $(#[$doc:meta])*
        $c_name:ident ($($arg:ident: $arg_ty:ty),*) -> $ret:ty => |$raw_var:ident| $convert:expr;
        as $rust_name:ident
    ) => {
        $(#[$doc])*
        pub fn $rust_name(&mut self, $($arg: $arg_ty),*) -> Result<$ret, SolverError> {
            unsafe {
                let $raw_var = bindings::$c_name(self.inner.as_ptr() $(, $arg.into())*);
                self.error()?;
                Ok($convert)
            }
        }
    };
}

impl GlucoseSolver {
    fn error_msg(code: i32) -> Result<(), SolverError> {
        unsafe {
            let msg: *const ::std::os::raw::c_char = bindings::glucose_error_msg(code);
            let msg = std::ffi::CStr::from_ptr(msg);
            return Err(SolverError(msg.to_str().unwrap()));
        }
    }
    fn error(&mut self) -> Result<(), SolverError> {
        unsafe {
            let code = bindings::glucose_error(self.inner.as_mut());
            if code != 0 {
                return GlucoseSolver::error_msg(code);
            }
        }
        Ok(())
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
    glucose_opt_g_set!(
        spec_inc_reduce_db,
        i32,
        "Special increment for reduce DB\n\n# Arguments\n* `value` - must be non-negative"
    );
    glucose_opt_set!(lb_lbd_frozen_clause, i32, "Protect clauses if LBD decreases below this\n\n# Arguments\n* `value` - must be non-negative");
    glucose_opt_g_set!(chanseok_hack, bool, "Use Chanseok Oh strategy for LBD");
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
    glucose_opt_g_set!(glu_reduction, bool, "Glucose reduction strategy");
    glucose_opt_g_set!(luby_restart, bool, "Use Luby restart sequence");
    glucose_opt_g_set!(
        restart_inc,
        f64,
        "Restart interval increase factor\n\n# Arguments\n* `value` - must be >= 1.0"
    );
    glucose_opt_g_set!(
        luby_restart_factor,
        i32,
        "Luby restart factor\n\n# Arguments\n* `value` - must be positive"
    );
    glucose_opt_g_set!(
        randomize_phase_on_restarts,
        i32,
        "Randomization level on restarts (0-3)\n\n# Arguments\n* `value` - must be 0-3"
    );
    glucose_opt_g_set!(
        fixed_randomize_phase_on_restarts,
        bool,
        "Fix first 7 levels at random phase"
    );
    glucose_opt_g_set!(adapt, bool, "Adapt strategies after 100000 conflicts\n\n# Arguments\n* `value` - boolean (1=true, 0=false)");
    glucose_opt_g_set!(forceunsat, bool, "Force phase for UNSAT");
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
        unsafe {
            GlucoseSolver {
                inner: NonNull::new(bindings::glucose_new_solver()).unwrap(),
            }
        }
    }
    ffi_bind! {
        /// Add a new variable to the solver.
        glucose_new_var() -> i32;
        as new_var
    }


    /// Add a clause to the solver.
    pub fn add_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        unsafe {
            bindings::glucose_add_clause(self.inner.as_ptr(), clause.as_ptr(), clause.len() as u64);
        }
        self.error()?;
        Ok(())
    }
    ffi_bind! {
        /// Add the empty clause to the solver.
        glucose_add_empty_clause() -> i32;
        as add_empty_clause
    }

    ffi_bind! {
        /// Get the value of a literal.
        glucose_value(x: i32) -> i32;
        as value
    }

    ffi_bind! {
        /// Get the value of a literal in the model.
        glucose_model_value(x: i32) -> bool=>|v|v!=0;
        as model_value
    }



/// Solve the problem with assumptions.
    pub fn solve_assumps(&mut self, clause: &[i32],do_simp: bool,
        turn_off_simp: bool) -> Result<RawStatus, SolverError> {
        let status= unsafe {
            bindings::glucose_solve_assumps(self.inner.as_ptr(), clause.as_ptr(), clause.len() as u64,do_simp.into(),turn_off_simp.into())
        }.into();
        self.error()?;
        Ok(status)
    }
/// Solve the problem with limited.
    pub fn solve_limited(&mut self, clause: &[i32],do_simp: bool,
        turn_off_simp: bool) -> Result<RawStatus, SolverError> {
        let status= unsafe {
            bindings::glucose_solve_limited(self.inner.as_ptr(), clause.as_ptr(), clause.len() as u64,do_simp.into(),turn_off_simp.into())
        }.into();
        self.error()?;
        Ok(status)
    }
        /// Solve the problem with limited.

    pub fn glucose_solve_limited(&mut self, clause: &[i32],do_simp: bool,
        turn_off_simp: bool) -> Result<RawStatus, SolverError> {
        let status= unsafe {
            bindings::glucose_solve_limited(self.inner.as_ptr(), clause.as_ptr(), clause.len() as u64,do_simp.into(),turn_off_simp.into())
        }.into();
        self.error()?;
        Ok(status)
    }

    ffi_bind! {
        /// Solve the problem.
        glucose_solve(do_simp: bool, turn_off_simp: bool) -> i32;
        as solve
    }

    ffi_bind! {
        /// Perform variable elimination based simplification.
        glucose_eliminate(turn_off_elim: bool) -> i32;
        as eliminate
    }

    ffi_bind! {
        /// The current number of assigned literals.
        glucose_nassigns() -> i32;
        as nassigns
    }

    ffi_bind! {
        /// The current number of original clauses.
        glucose_nclauses() -> i32;
        as nclauses
    }

    ffi_bind! {
        /// The current number of learnt clauses.
        glucose_nlearnts() -> i32;
        as nlearnts
    }

    ffi_bind! {
        /// The current number of variables.
        glucose_nvars() -> i32;
        as nvars
    }

    ffi_bind! {
        /// The current number of free variables.
        glucose_nfree_vars() -> i32;
        as nfree_vars
    }

    ffi_bind! {
        /// Destroy the solver.
        glucose_destroy() -> ();
        as destroy
    }

    ffi_bind! {
        /// Check if the solver is okay.
        glucose_okay() -> i32;
        as okay
    }
}

impl SatSolver for GlucoseSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        GlucoseSolver::add_clause(self, clause)?;
        Ok(())
    }
    fn solve_sat(&mut self) -> Result<RawStatus, SolverError> {
        self.eliminate(true);
        self.solve_limited(&[], true, false)
    }

    fn model(&mut self) -> Result<Vec<i32>, SolverError> {
       let mut model =vec![];
        for lit in 1..=self.nvars()? {
            if self.model_value(lit)? {
                model.push(lit);
            } else {
                model.push(-lit);
            }
        }
        Ok(model)
    }
}
impl Drop for GlucoseSolver {
    fn drop(&mut self) {
        unsafe {
            bindings::glucose_destroy(self.inner.as_ptr());
        }
    }
}
