//! The `picosat` module provides access to the `PicoSATSolver`.
//!
//! This module is enabled when the `picosat` feature is activated.
//!
//! # Overview
//! The `PicoSATSolver` struct acts as a wrapper for the [PicoSAT](https://fmv.jku.at/picosat/) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `picosat` module, ensure the `picosat` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! satgalaxy = { version = "x.y.z", features = ["picosat"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod binding {
     include!("../../bindings/picosat_bindings.rs");
}

use std::{collections::HashSet, fmt::Display, os::raw, ptr::NonNull};

use crate::{errors::SolverError, solver::RawStatus};

use super::{MusSolver, MusStatus, SatSolver};

macro_rules! ffi_bind {
    (
        $(#[$doc:meta])*
        $c_name:ident ($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
        as $rust_name:ident
    ) => {
        $(#[$doc])*
        pub fn $rust_name(&mut self, $($arg: $arg_ty),*) -> Result<$ret, SolverError> {
            unsafe {
                let ret = binding::$c_name(self.inner.as_ptr() $(, $arg.into())*);
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
                let $raw_var = binding::$c_name(self.inner.as_ptr() $(, $arg.into())*);
                self.error()?;
                Ok($convert)
            }
        }
    };
}

fn ptr_to_vec<T: Display + PartialEq + std::cmp::PartialEq<i32>>(ptr: *const T) -> Vec<T> {
    let mut vec = Vec::new();
    let mut curr = ptr;
    let mut v = unsafe { curr.read() };

    while !curr.is_null() && v != 0 {
        unsafe {
            vec.push(v);
            curr = curr.offset(1);
            v = curr.read();
        }
    }
    vec
}

/// `PicoSATSolver` is a wrapper for the [PicoSAT](https://github.com/arminbiere/picosat) Solver .
/// It also allows creating a `PicoSAT_Solver` instance for more low-level operations.
/// This struct is only available when the `picosat` feature is enabled.
/// # Example
/// ```rust
/// use satgalaxy::solver::{PicoSATSolver, SatStatus, SatSolver};
/// let mut solver = PicoSATSolver::new();
///     solver.add_clause(&vec![1, 2]);
///     solver.add_clause(&vec![-1, -2]);
///     solver.add_clause(&vec![3]);
///
/// match solver.solve() {
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
///  To use the `PicoSATSolver`, ensure the `picosat` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  satgalaxy = { version = "x.y.z", features = ["picosat"] }
#[derive(Debug, Clone)]
pub struct PicoSATSolver {
    inner: NonNull<binding::PicoSATSolver>,
    clauses: Vec<Vec<i32>>,
    vars: i32,
}
impl Default for PicoSATSolver {
    fn default() -> Self {
        Self::new()
    }
}
impl PicoSATSolver {
    pub fn new() -> Self {
        unsafe {
            PicoSATSolver {
                inner: NonNull::new(binding::picosat_s_init()).unwrap(),
                clauses: Vec::new(),
                vars: 0,
            }
        }
    }
    fn error(&self) -> Result<(), SolverError> {
        unsafe {
            let code = binding::picosat_s_error(self.inner.as_ptr());
            if code != 0 {
                let msg = binding::picosat_s_errmsg(code);
                let msg = std::ffi::CStr::from_ptr(msg);
                return Err(SolverError(msg.to_str().unwrap()));
            }
        }
        Ok(())
    }

    /// Add a clause to the solver.
    ///
    /// # Arguments
    /// * `clause` - A reference to a vector of literals representing the clause.
    fn add_inner_clause(&self, clause: &[i32]) -> Result<(), SolverError> {
        unsafe {
            binding::picosat_s_add_lits(self.inner.as_ptr(), clause.as_ptr(), clause.len());
        }
        self.error()?;
        Ok(())
    }

    fn add_inner(&self, lit: i32) -> Result<(), SolverError> {
        unsafe {
            binding::picosat_s_add(self.inner.as_ptr(), lit);
        }
        self.error()?;
        Ok(())
    }

    // ffi_bind! {
    //     /// Configures initial solver settings
    //     ///
    //     /// These functions must be called immediately after picosat_s_init and before adding literals.
    //     ///
    //     /// # Arguments
    //     /// * `file` - Output file for solver messages (default: stdout)
    //     picosat_s_set_output (file: *mut libc::FILE) -> ();
    //     as set_output
    // }

    ffi_bind! {
        /// Measure all time spent in all calls in the solver
        ///
        /// By default only the time spent in 'picosat_s_sat' is measured.
        ///
        /// # Note
        /// Enabling this may significantly increase time for adding large CNFs due to getrusage calls.
        picosat_s_measure_all_calls () -> ();
        as measure_all_calls
    }

    ffi_bind! {
        /// Sets the prefix for verbose messages and statistics
        ///
        /// # Arguments
        /// * `prefix` - String prefix for messages (default: "c ")
        picosat_s_set_prefix (prefix: *const raw::c_char) -> ();
        as set_prefix
    }

    ffi_bind! {
        /// Sets the verbosity level for the solver
        ///
        /// # Arguments
        /// * `new_verbosity_level` - Verbosity level (1+ for detailed progress reports)
        ///
        /// # Note
        /// Reports are printed to the output file set by `set_output`, prefixed by `set_prefix`
        picosat_s_set_verbosity (new_verbosity_level: i32) -> ();
        as set_verbosity
    }

    ffi_bind! {
        /// Disables or enables preprocessing
        ///
        /// # Arguments
        /// * `new_plain_value` - Non-zero to disable preprocessing (plain solving), zero to enable
        ///
        /// # Note
        /// Currently affects only failed literal probing.
        picosat_s_set_plain (new_plain_value: i32) -> ();
        as set_plain
    }

    ffi_bind! {
        /// Sets the default initial phase for decision variables
        ///
        /// # Arguments
        /// * `phase` - Phase value: 0 (false), 1 (true), 2 (Jeroslow-Wang, default), 3 (random)
        ///
        /// # Note
        /// After first assignment, variables reuse their previous value for decisions.
        picosat_s_set_global_default_phase (phase: i32) -> ();
        as set_global_default_phase
    }

    ffi_bind! {
        /// Sets the initial phase for a specific variable
        ///
        /// # Arguments
        /// * `lit` - Literal to set the phase for
        /// * `phase` - Phase value: negative (false), positive (true), 0 (use global default phase)
        ///
        /// # Note
        /// Forced assignments override this phase for decision variables.
        picosat_s_set_default_phase_lit (lit: i32, phase: i32) -> ();
        as set_default_phase_lit
    }

    ffi_bind! {
        /// Resets all variable phases to their default state
        ///
        picosat_s_reset_phases () -> ();
        as reset_phases
    }

    ffi_bind! {
        /// Erases variable scores, keeping learned clauses
        ///
        ///
        /// # Note
        /// Incremental mode may differ from a fresh CNF due to retained clauses.
        picosat_s_reset_scores () -> ();
        as reset_scores
    }

    ffi_bind! {
        /// Removes learned clauses
        ///
        /// # Arguments
        /// * `percentage` - Percentage of large learned clauses to remove (100% removes all)
        picosat_s_remove_learned (percentage: u32) -> ();
        as remove_learned
    }

    ffi_bind! {
        /// Marks a literal as more important for decisions
        ///
        /// # Arguments
        /// * `lit` - Literal to mark as more important
        ///
        /// # Note
        /// Default is all variables marked as indifferent.
        picosat_s_set_more_important_lit (lit: i32) -> ();
        as set_more_important_lit
    }

    ffi_bind! {
        /// Marks a literal as less important for decisions
        ///
        /// # Arguments
        /// * `lit` - Literal to mark as less important
        ///
        /// # Note
        /// Default is all variables marked as indifferent.
        picosat_s_set_less_important_lit (lit: i32) -> ();
        as set_less_important_lit
    }

    ffi_bind! {
        /// Sets the seed for the random number generator
        ///
        /// # Arguments
        /// * `random_number_generator_seed` - Seed value for the random number generator
        ///
        /// # Note
        /// Useful for benchmarking different parameter sets, less effective for industrial examples.
        picosat_s_set_seed (random_number_generator_seed: u32) -> ();
        as set_seed
    }

    ffi_bind! {
        /// Enables proof trace generation
        ///
        ///
        /// # Note
        /// - Not needed for `set_incremental_rup_file`
        /// - Trace generation may not be included if compiled with full optimization
        ///
        /// # Returns
        /// `true` if trace generation is supported, `false` otherwise
        picosat_s_enable_trace_generation () -> bool => |res| res != 0;
        as enable_trace_generation
    }

    // ffi_bind! {
    //     /// Sets a file for incremental RUP proof trace dumping
    //     ///
    //     /// # Arguments
    //     /// * `file` - Output file for the RUP trace
    //     /// * `m` - Maximum number of variables
    //     /// * `n` - Number of original clauses
    //     ///
    //     /// # Note
    //     /// - Reduces memory usage
    //     /// - Dumped clauses may not be in the clausal core
    //     picosat_s_set_incremental_rup_file (file: *mut libc::FILE, m: i32, n: i32) -> ();
    //     as set_incremental_rup_file
    // }

    ffi_bind! {
        /// Saves original clauses for partial dereferencing
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        picosat_s_save_original_clauses () -> ();
        as save_original_clauses
    }

    ffi_bind! {
        /// Allocates and returns the next unused variable index
        ///
        ///
        /// # Note
        /// The variable is treated as used in future solver calls
        ///
        /// # Returns
        /// The next available variable index
        picosat_s_inc_max_var () -> i32;
        as inc_max_var
    }

    ffi_bind! {
        /// Push semantics for PicoSAT (creates new context)
        ///
        ///
        /// # Returns
        /// The index of the literal that assumes this context
        picosat_s_push () -> i32;
        as push
    }

    ffi_bind! {
        /// Checks if a context literal failed
        ///
        /// # Arguments
        /// * `lit` - Literal to check (generated by `push`)
        ///
        /// # Returns
        /// `true` if the context failed, `false` otherwise
        picosat_s_failed_context (lit: i32) -> bool => |res| res != 0;
        as failed_context
    }

    ffi_bind! {
        /// Gets the literal for the current context
        ///
        ///
        /// # Returns
        /// Literal that assumes the current context or zero for outer context
        picosat_s_context () -> i32;
        as context
    }

    ffi_bind! {
        /// Pop semantics for PicoSAT (closes current context)
        ///
        ///
        /// # Returns
        /// Literal for the new outer context or zero if outermost
        picosat_s_pop () -> i32;
        as pop
    }

    ffi_bind! {
        /// Forces immediate removal of satisfied clauses
        ///
        /// # Note
        /// - Called internally after sufficient units are learned
        /// - Retains learned clauses involving outer contexts
        picosat_s_simplify () -> ();
        as simplify
    }

    ffi_bind! {
        /// Optimizes variable table size
        ///
        /// # Arguments
        /// * `max_idx` - Estimated maximum variable index
        ///
        /// # Note
        /// Has the same effect as `inc_max_var` but optimizes allocation
        picosat_s_adjust (max_idx: i32) -> ();
        as adjust
    }

    ffi_bind! {
        /// Gets the number of variables
        ///
        ///
        /// # Returns
        /// Number of variables (p cnf <m> n)
        picosat_s_variables () -> i32;
        as variables
    }

    ffi_bind! {
        /// Gets the number of original clauses
        ///
        ///
        /// # Returns
        /// Number of original clauses (p cnf m <n>)
        picosat_s_added_original_clauses () -> i32;
        as added_original_clauses
    }

    ffi_bind! {
        /// Gets maximum memory allocated
        ///
        /// # Returns
        /// Maximum memory allocated in bytes
        picosat_s_max_bytes_allocated () -> usize;
        as max_bytes_allocated
    }

    ffi_bind! {
        /// Prints solver statistics to output file
        ///
        picosat_s_stats () -> ();
        as stats
    }

    ffi_bind! {
        /// Gets the number of propagations
        ///
        ///
        /// # Returns
        /// Number of propagations
        picosat_s_propagations () -> u64;
        as propagations
    }

    ffi_bind! {
        /// Gets the number of decisions
        ///
        ///
        /// # Returns
        /// Number of decisions
        picosat_s_decisions () -> u64;
        as decisions
    }

    ffi_bind! {
        /// Gets the number of visits
        ///
        /// # Returns
        /// Number of visits
        picosat_s_visits () -> u64;
        as visits
    }

    ffi_bind! {
        /// Gets time spent in solver
        ///
        ///
        /// # Returns
        /// Time spent in library calls or SAT solving
        picosat_s_seconds () -> f64;
        as seconds
    }

    ffi_bind! {
        /// Adds a literal to the solver
        ///
        /// # Arguments
        /// * `lit` - Literal to add
        ///
        /// # Note
        /// Adding a literal resets the previous assignment
        ///
        /// # Returns
        /// Original clause index for the added literal
        picosat_s_add (lit: i32) -> i32;
        as add
    }

    // ffi_bind! {
    //     /// Prints the CNF to a file in DIMACS format
    //     ///
    //     /// # Arguments
    //     /// * `file` - Output file for the CNF
    //     picosat_s_print (file: *mut libc::FILE) -> ();
    //     as print
    // }

    ffi_bind! {
        /// Adds an assumption for the next SAT call
        ///
        /// # Arguments
        /// * `lit` - Literal to assume (interpreted as unit clause)
        ///
        /// # Note
        /// - Assumptions are valid only for the next SAT call
        /// - Cleared after SAT call unless reassumed
        picosat_s_assume (lit: i32) -> ();
        as assume
    }

    ffi_bind! {
        /// Adds a literal to the global all-different constraint
        ///
        /// # Arguments
        /// * `lit` - Literal to add to the ADC
        ///
        /// # Note
        /// Only one global ADC is supported
        picosat_s_add_ado_lit (lit: i32) -> ();
        as add_ado_lit
    }

    ffi_bind! {
        /// Runs the main SAT solving routine
        ///
        /// # Arguments
        /// * `decision_limit` - Maximum number of decisions (negative for no limit)
        ///
        picosat_s_sat (decision_limit: i32) -> RawStatus;
        as sat
    }

    ffi_bind! {
        /// Sets a propagation limit for SAT solving
        ///
        /// # Arguments
        /// * `limit` - Maximum number of propagations
        ///
        /// # Note
        /// Must be called after initialization and before SAT solving
        picosat_s_set_propagation_limit (limit: u64) -> ();
        as set_propagation_limit
    }

    ffi_bind! {
        /// Returns the result of the last SAT call
        ///
        /// # Returns
        /// Last SAT result or 0 if not called
        picosat_s_res () -> RawStatus;
        as res
    }

    ffi_bind! {
        /// Dereferences a literal to get its assignment
        ///
        /// # Arguments
        /// * `lit` - Literal to dereference
        ///
        /// # Returns
        /// * `Some(true)` if literal is true
        /// * `Some(false)` if literal is false
        /// * `None` if literal is unassigned
        picosat_s_deref (lit: i32) -> Option<bool> => |res| {
            match res {
                1 => Some(true),
                -1 => Some(false),
                _ => None,
            }
        };
        as deref
    }

    ffi_bind! {
        /// Checks if a literal is forced at the top level
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Note
        /// Does not require SAT solving and does not reset incremental usage
        ///
        /// # Returns
        /// * `Some(true)` if literal is forced true
        /// * `Some(false)` if literal is forced false
        /// * `None` if not forced
        picosat_s_deref_toplevel (lit: i32) -> Option<bool> => |res| {
            match res {
                1 => Some(true),
                -1 => Some(false),
                _ => None,
            }
        };
        as deref_toplevel
    }

    ffi_bind! {
        /// Gets partial satisfying assignment for original clauses
        ///
        /// # Arguments
        /// * `lit` - Literal to dereference
        ///
        /// # Note
        /// Requires `save_original_clauses` to be called after initialization
        ///
        /// # Returns
        /// * `Some(true)` if literal is true in partial assignment
        /// * `Some(false)` if literal is false in partial assignment
        /// * `None` if not assigned
        picosat_s_deref_partial (lit: i32) -> Option<bool> => |res| {
            match res {
                1 => Some(true),
                -1 => Some(false),
                _ => None,
            }
        };
        as deref_partial
    }

    ffi_bind! {
        /// Checks if the CNF is inconsistent (has empty clause)
        ///
        /// # Returns
        /// `true` if inconsistent (has empty clause), `false` otherwise
        picosat_s_inconsistent () -> bool => |res| res != 0;
        as is_inconsistent
    }

    ffi_bind! {
        /// Checks if a literal is a failed assumption
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Note
        /// Only valid while current assumptions are active
        ///
        /// # Returns
        /// `true` if the literal is a failed assumption, `false` otherwise
        picosat_s_failed_assumption (lit: i32) -> bool => |res| res != 0;
        as failed_assumption
    }

    ffi_bind! {
        /// Gets a list of failed assumptions
        ///
        /// # Note
        /// - Valid until next SAT or failed assumptions call
        /// - Only meaningful if SAT returned UNSATISFIABLE
        ///
        /// # Returns
        ///
        picosat_s_failed_assumptions () -> Vec<i32> => |res| {
            ptr_to_vec(res)
        };
        as failed_assumptions
    }

    ffi_bind! {
        /// Computes a minimized list of failed assumptions
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        /// * `state` - User-defined state for callback
        /// * `callback` - Function called for each simplified assumption set
        /// * `fix` - Non-zero to permanently assign assumptions
        ///
        /// # Note
        /// - Only meaningful if SAT returned UNSATISFIABLE
        /// - Valid until next SAT or MUS call
        ///
        /// # Returns
        /// Pointer to zero-terminated minimized array of failed assumptions
        picosat_s_mus_assumptions (
            state: *mut raw::c_void,
            callback: Option<unsafe extern "C" fn(*mut raw::c_void, *const i32)>,
            fix: i32
        ) -> Vec<i32> =>|res|ptr_to_vec(res);


        as mus_assumptions
    }

    ffi_bind! {
        /// Computes maximal satisfiable subset of assumptions
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        ///
        /// # Note
        /// - Requires assumptions set and SAT called
        /// - Reassumes all assumptions before returning
        ///
        /// # Returns
        /// Pointer to zero-terminated array of consistent assumptions
        picosat_s_maximal_satisfiable_subset_of_assumptions () -> *const i32;
        as maximal_satisfiable_subset_of_assumptions
    }

    ffi_bind! {
        /// Iterates over maximal satisfiable subsets of assumptions
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        ///
        /// # Note
        /// - Adds blocking clauses that alter CNF
        /// - Requires assumptions set via `assume`
        ///
        /// # Returns
        /// Pointer to zero-terminated array or NULL if none remain
        picosat_s_next_maximal_satisfiable_subset_of_assumptions () -> *const i32;
        as next_maximal_satisfiable_subset_of_assumptions
    }

    ffi_bind! {
        /// Iterates over minimal correcting assumption sets
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        ///
        /// # Note
        /// Each assumed literal appears once in the result
        ///
        /// # Returns
        /// Pointer to zero-terminated array of minimal correcting assumptions
        picosat_s_next_minimal_correcting_subset_of_assumptions () -> *const i32;
        as next_minimal_correcting_subset_of_assumptions
    }

    ffi_bind! {
        /// Computes union of all minimal correcting assumption sets (HUMUS)
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSATSolver
        /// * `callback` - Function called after each minimal set
        /// * `state` - User-defined state for callback
        ///
        /// # Note
        /// - Renders CNF inconsistent after call
        /// - Requires solver reset after use
        ///
        /// # Returns
        /// Pointer to zero-terminated array of assumptions in HUMUS
        picosat_s_humus (
            callback: Option<unsafe extern "C" fn(*mut raw::c_void, i32, i32)>,
            state: *mut raw::c_void
        ) -> Vec<i32> =>|res|ptr_to_vec(res);
        as humus
    }

    ffi_bind! {
        /// Checks if old variable assignments changed between SAT calls
        ///
        ///
        /// # Note
        /// - Valid until add/assume/SAT called
        /// - May return true even if no change occurred
        ///
        /// # Returns
        /// `true` if assignments might have changed, `false` otherwise
        picosat_s_changed () -> bool => |res| res != 0;
        as changed
    }

    ffi_bind! {
        /// Checks if original clause is in clausal core
        ///
        /// # Arguments
        /// * `solver` - Pointer to the PicoSAT solver
        /// * `i` - Clause index (0 <= i < added_original_clauses)
        ///
        /// # Note
        /// Requires trace generation enabled
        ///
        /// # Returns
        /// `true` if clause is in core, `false` otherwise
        picosat_s_coreclause (i: i32) -> bool => |res| res != 0;
        as coreclause
    }

    ffi_bind! {
        /// Checks if literal is in variable core
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Note
        /// Requires trace generation enabled
        ///
        /// # Returns
        /// `true` if literal is in core, `false` otherwise
        picosat_s_corelit (lit: i32) -> bool => |res| res != 0;
        as corelit
    }

    // ffi_bind! {
    //     /// Writes clausal core to file
    //     ///
    //     /// # Arguments
    //     /// * `core_file` - Output file for clausal core
    //     ///
    //     /// # Note
    //     /// Requires trace generation enabled
    //     picosat_s_write_clausal_core (core_file: *mut libc::FILE) -> ();
    //     as write_clausal_core
    // }

    // ffi_bind! {
    //     /// Writes compact proof trace to file
    //     ///
    //     /// # Arguments
    //     /// * `trace_file` - Output file for proof trace
    //     ///
    //     /// # Note
    //     /// Requires trace generation enabled
    //     picosat_s_write_compact_trace (trace_file: *mut libc::FILE) -> ();
    //     as write_compact_trace
    // }

    // ffi_bind! {
    //     /// Writes extended proof trace to file
    //     ///
    //     /// # Arguments
    //     /// * `trace_file` - Output file for proof trace
    //     ///
    //     /// # Note
    //     /// Requires trace generation enabled
    //     picosat_s_write_extended_trace (trace_file: *mut libc::FILE) -> ();
    //     as write_extended_trace
    // }

    // ffi_bind! {
    //     /// Writes RUP trace to file
    //     ///
    //     /// # Arguments
    //     /// * `trace_file` - Output file for RUP trace
    //     ///
    //     /// # Note
    //     /// - Requires trace generation enabled
    //     /// - Includes only learned core clauses
    //     picosat_s_write_rup_trace (trace_file: *mut raw::) -> ();

    //     as write_rup_trace
    // }

    ffi_bind! {
        /// Checks if literal was used in resolution
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Note
        /// Core literals are a subset of used literals
        ///
        /// # Returns
        /// `true` if literal was used, `false` otherwise
        picosat_s_usedlit (lit: i32) -> bool => |res| res != 0;
        as usedlit
    }
    ffi_bind! {
        /// Enters the PicoSAT solver context.
        picosat_s_enter ()->(); as enter
    }
    ffi_bind! {
        /// Exits the PicoSAT solver context.
        picosat_s_leave ()->(); as leave
    }

    fn core_extraction(
        &mut self,
        max_rounds: i32,
        min_rounds: i32,
        max_non_red_rounds: i32,
        redundant_indices: &mut HashSet<usize>,
    ) -> Result<RawStatus, SolverError> {
        let mut non_red_count = 0;
        let mut n = self.clauses.len();

        for round in 1..=max_rounds {
            self.set_seed(round as u32)?;

            // 添加子句
            for (i, clause) in self.clauses.iter().enumerate() {
                if redundant_indices.contains(&i) {
                    self.add_inner_clause(&[1, -1])?;
                } else {
                    self.add_inner_clause(clause)?;
                }
            }

            let res: RawStatus = self.sat(-1)?;
            if res == RawStatus::Satisfiable {
                return Ok(res);
            }
            for i in 0..self.clauses.len() {
                if redundant_indices.contains(&i) {
                    continue;
                }
                if !self.coreclause(i as i32)? {
                    redundant_indices.insert(i);
                }
            }
            let old_n = n;
            n = self.clauses.len() - redundant_indices.len();
            unsafe {
                binding::picosat_s_reset(self.inner.as_ptr());
                self.inner = NonNull::new(binding::picosat_s_init()).unwrap();
            }
            if round >= min_rounds {
                let red = old_n - n;
                if red < 10 && (100 * red + 99) / old_n < 2 {
                    non_red_count += 1;
                    if non_red_count > max_non_red_rounds {
                        break;
                    }
                } else {
                    non_red_count = 0;
                }
                if non_red_count > max_non_red_rounds {
                    break;
                }
            }
            if round < max_rounds {
                self.enable_trace_generation()?;
            }
        }

        Ok(RawStatus::Unsatisfiable)
    }
}

impl SatSolver for PicoSATSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        self.add_inner_clause(clause)
    }

    fn solve_sat(&mut self) -> Result<RawStatus, SolverError> {
        self.sat(-1)
    }

    fn model(&mut self) -> Result<Vec<i32>, SolverError> {
        let mut model = Vec::new();
        for v in 0..self.variables()? {
            let lit = v + 1;
            let assum = self.deref(lit)?;
            if let Some(true) = assum {
                model.push(lit);
            }
        }
        Ok(model)
    }
}

impl MusSolver for PicoSATSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        self.vars = clause
            .iter()
            .map(|lit| lit.abs())
            .max()
            .unwrap_or(0)
            .max(self.vars);
        self.clauses.push(clause.to_vec());
        Ok(())
    }

    fn solve_mus(&mut self) -> Result<MusStatus, SolverError> {
        let mut redundant_indices: HashSet<usize> = HashSet::new();
        if self.enable_trace_generation()? {
            let res = self.core_extraction(100, 3, 3, &mut redundant_indices)?;
            if res == RawStatus::Satisfiable {
                return Ok(MusStatus::Satisfiable);
            }
        }
        let idx: i32 = self.vars + 1;
        for (i, clause) in self.clauses.iter().enumerate() {
            if redundant_indices.contains(&i) {
                continue;
            }
            self.add_inner(-(idx + i as i32))?;
            self.add_inner_clause(&clause)?;
        }

        for i in 0..self.clauses.len() {
            if !redundant_indices.contains(&i) {
                self.assume(idx + i as i32)?;
            }
        }
        let status = self.sat(-1)?;
        match status {
            RawStatus::Satisfiable => Ok(MusStatus::Satisfiable),
            RawStatus::Unsatisfiable => {
                redundant_indices.extend(0..self.clauses.len());
                let mut mus = unsafe {
                    binding::picosat_s_mus_assumptions(
                        self.inner.as_ptr(),
                        std::ptr::null_mut(),
                        None,
                        1,
                    )
                };
                self.error()?;
                while !mus.is_null() {
                    let assumption = unsafe { *mus };
                    if assumption == 0 {
                        break;
                    }
                    redundant_indices.remove(&((assumption - idx) as usize));
                    mus = unsafe { mus.offset(1) };
                }
                let mus: Vec<usize> = (0..self.clauses.len())
                    .filter(|i| !redundant_indices.contains(&i))
                    .collect::<Vec<usize>>();
                Ok(MusStatus::Unsatisfiable(mus))
            }
            RawStatus::Unknown => Ok(MusStatus::Unknown),
        }
    }
}

impl Drop for PicoSATSolver {
    fn drop(&mut self) {
        unsafe {
            binding::picosat_s_reset(self.inner.as_ptr());
        }
    }
}
