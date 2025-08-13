//! The `cadical` module provides access to the `CaDiCaLSolver`.
//!
//! This module is enabled when the `minisat` feature is activated.
//!
//! # Overview
//! The `CaDiCaLSolver` struct acts as a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver, allowing users to
//! leverage its functionality for solving SAT problems.
//!
//! # Usage
//! To use the `cadical` module, ensure the `cadical` feature is enabled in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! satgalaxy = { version = "x.y.z", features = ["cadical"] }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod binding {
    include!("../../bindings/cadical_bindings.rs");
}

use std::{ffi::c_char, ptr::NonNull};

use crate::{errors::SolverError, solver::RawStatus};

use super::{SatSolver};

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

/// `CaDiCaLSolver` is a wrapper for the [CaDiCaL](https://github.com/arminbiere/cadical) Solver .
/// It also allows creating a `CaDiCaL_Solver` instance for more low-level operations.
/// This struct is only available when the `cadical` feature is enabled.
/// # Example
/// ```rust
/// use satgalaxy::solver::{CaDiCaLSolver, SatStatus, SatSolver};
/// let mut solver = CaDiCaLSolver::new();
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
///  To use the `CaDiCaLSolver`, ensure the `cadical` feature is enabled in your `Cargo.toml`:
///  ```toml
///  [dependencies]
///  satgalaxy = { version = "x.y.z", features = ["cadical"] }
#[derive(Debug, Clone)]
pub struct CaDiCaLSolver{
    inner: NonNull<binding::CaDiCaLSolver>,
}
impl Default for CaDiCaLSolver {
    fn default() -> Self {
        Self::new()
    }
}
impl CaDiCaLSolver {
    pub fn new() -> Self {
        unsafe { CaDiCaLSolver { inner: NonNull::new(binding::cadical_new_solver()).unwrap() } }
    }
    fn error(&mut self) -> Result<(), SolverError> {
        unsafe {
            let code = binding::cadical_error(self.inner.as_ptr());
            if code != 0 {
                let msg = binding::cadical_error_message(code);
                let msg = std::ffi::CStr::from_ptr(msg);
                return Err(SolverError(msg.to_str().unwrap()));
            }
        }
        Ok(())
    }

    /// Add a clause to the solver.
    ///
    /// # Arguments
    /// * `ps` - Pointer to an array of literals
    /// * `length` - Length of the array
    pub fn add_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        unsafe {
            binding::cadical_add_clause(self.inner.as_ptr(), clause.as_ptr(), clause.len());
        }
        self.error()?;
        Ok(())
    }

    ffi_bind! {
        /// Add an empty clause to the solver.
        cadical_add_empty_clause() -> ();
        as add_empty_clause
    }

    ffi_bind! {
        /// Add a literal to the current clause.
        ///
        /// # Arguments
        /// * `lit` - Literal to add
        cadical_add(lit: i32) -> ();
        as add
    }

    ffi_bind! {
        /// Assume a literal for the next solve call.
        ///
        /// # Arguments
        /// * `lit` - Literal to assume
        cadical_assume(lit: i32) -> ();
        as assume
    }

    ffi_bind! {
        /// Solve the formula under current assumptions.
        ///
        /// # Returns
        /// `SolveResult` enum:
        /// * `Satisfiable` (10)
        /// * `Unsatisfiable` (20)
        /// * `Unknown` (30)
        cadical_solve() -> RawStatus;
        as solve
    }

    ffi_bind! {
        /// Get the value of a literal.
        ///
        /// # Arguments
        /// * `lit` - Literal to query
        ///
        /// # Returns
        /// * Positive value if true
        /// * Negative value if false
        cadical_val(lit: i32) -> i32;
        as val
    }

    ffi_bind! {
        /// Check if a literal is in the unsatisfiable core.
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Returns
        /// `true` if the literal is in the core, `false` otherwise.
        cadical_failed(lit: i32) -> bool=>|v|v!=0;

        as failed
    }

    ffi_bind! {
        /// Add a literal to the constraint clause.
        cadical_constrain(lit: i32) -> ();
        as constrain
    }

    ffi_bind! {
        /// Check if the constraint was used to prove unsatisfiability.
        ///
        /// # Returns
        /// `true` if the constraint was used, `false` otherwise.
        cadical_constraint_failed() -> bool=>|v|v!=0;
        as constraint_failed
    }

    /// Set a solver option.
    ///
    /// # Arguments
    /// * `name` - Option name
    /// * `val` - Option value
    ///
    /// # Returns
    /// `true` if successful, `false` otherwise.
    pub fn set_option(&mut self, name: &str, val: i32) -> Result<bool, SolverError> {
        let name = name.as_bytes();
        let name = name.as_ptr() as *const c_char;
        unsafe {
            binding::cadical_set_option(self.inner.as_ptr(), name, val);
        }
        self.error()?;
        Ok(true)
    }

    ffi_bind! {
        /// Set a search limit.
        ///
        /// # Arguments
        /// * `name` - Limit type ("conflicts", "decisions", etc.)
        /// * `limit` - Limit value
        ///
        /// # Returns
        /// `true` if successful, `false` otherwise.
        cadical_limit(name: *const c_char, limit: i32) -> bool=>|v|v!=0;
        as limit
    }

    /// Get the current value of an option.
    ///
    /// # Arguments
    /// * `name` - Option name
    ///
    /// # Returns
    /// Current value of the option.
    pub fn get_option(&mut self, name: &str) -> Result<i32, SolverError> {
        let name = name.as_bytes();
        let name = name.as_ptr() as *const c_char;
        let ret = unsafe { binding::cadical_get_option(self.inner.as_ptr(), name) };
        self.error()?;
        Ok(ret)
    }

    ffi_bind! {
        /// Print solver statistics.
        cadical_print_statistics() -> ();
        as print_statistics
    }

    ffi_bind! {
        /// Get the number of active variables.
        ///
        /// # Returns
        /// Number of active variables.
        cadical_active() -> i64;
        as active
    }

    ffi_bind! {
        /// Get the number of irredundant clauses.
        ///
        /// # Returns
        /// Number of irredundant clauses.
        cadical_irredundant() -> i64;
        as irredundant
    }

    ffi_bind! {
        /// Check if a literal is fixed at root level.
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Returns
        /// * `1` if literal is implied
        /// * `-1` if negation is implied
        /// * `0` otherwise
        cadical_fixed(lit: i32) -> i32;
        as fixed
    }

    ffi_bind! {
        /// Conclude solving process.
        cadical_conclude() -> ();
        as conclude
    }

    ffi_bind! {
        /// Terminate solving asynchronously.
        cadical_terminate() -> ();
        as terminate
    }
    ffi_bind! {
        /// Get the number of variables.
        cadical_vars() -> i32;
        as vars
    }

    ffi_bind! {
        /// Freeze a literal.
        ///
        /// # Arguments
        /// * `lit` - Literal to freeze
        cadical_freeze(lit: i32) -> ();
        as freeze
    }

    ffi_bind! {
        /// Check if a literal is frozen.
        ///
        /// # Arguments
        /// * `lit` - Literal to check
        ///
        /// # Returns
        /// `true` if frozen, `false` otherwise.
        cadical_frozen(lit: i32) -> bool=>|v|v!=0;
        as frozen
    }

    ffi_bind! {
        /// Unfreeze a literal.
        ///
        /// # Arguments
        /// * `lit` - Literal to unfreeze
        cadical_melt(lit: i32) -> ();
        as melt
    }

    ffi_bind! {
        /// Execute preprocessing.
        ///
        /// # Returns
        /// `RawStatus` status after preprocessing.
        cadical_simplify() -> RawStatus;
        as simplify
    }
    ffi_bind! {
    /// Allocate clauses in arena
     cadical_set_opt_arena(arena: bool) -> bool => |v|v!=0; as set_opt_arena }

    ffi_bind! {
    ///
     cadical_set_opt_arenacompact(arenacompact: i32) -> bool => |v|v!=0; as set_opt_arenacompact }
    ffi_bind! {
    ///
     cadical_set_opt_arenasort(arenasort: i32) -> bool => |v|v!=0; as set_opt_arenasort }
    ffi_bind! {
    ///
     cadical_set_opt_arenatype(arenatype: i32) -> bool => |v|v!=0; as set_opt_arenatype }
    ffi_bind! {
    ///
     cadical_set_opt_binary(binary: i32) -> bool => |v|v!=0; as set_opt_binary }
    ffi_bind! {
    ///
     cadical_set_opt_block(block: i32) -> bool => |v|v!=0; as set_opt_block }
    ffi_bind! {
    ///
     cadical_set_opt_blockmaxclslim(blockmaxclslim: i32) -> bool => |v|v!=0; as set_opt_blockmaxclslim }
    ffi_bind! {
    ///
     cadical_set_opt_blockminclslim(blockminclslim: i32) -> bool => |v|v!=0; as set_opt_blockminclslim }
    ffi_bind! {
    ///
     cadical_set_opt_blockocclim(blockocclim: i32) -> bool => |v|v!=0; as set_opt_blockocclim }
    ffi_bind! {
    ///
     cadical_set_opt_bump(bump: i32) -> bool => |v|v!=0; as set_opt_bump }
    ffi_bind! {
    ///
     cadical_set_opt_bumpreason(bumpreason: i32) -> bool => |v|v!=0; as set_opt_bumpreason }
    ffi_bind! {
    ///
     cadical_set_opt_bumpreasondepth(bumpreasondepth: i32) -> bool => |v|v!=0; as set_opt_bumpreasondepth }
    ffi_bind! {
    ///
     cadical_set_opt_check(check: i32) -> bool => |v|v!=0; as set_opt_check }
    ffi_bind! {
    ///
     cadical_set_opt_checkassumptions(checkassumptions: i32) -> bool => |v|v!=0; as set_opt_checkassumptions }
    ffi_bind! {
    ///
     cadical_set_opt_checkconstraint(checkconstraint: i32) -> bool => |v|v!=0; as set_opt_checkconstraint }
    ffi_bind! {
    ///
     cadical_set_opt_checkfailed(checkfailed: i32) -> bool => |v|v!=0; as set_opt_checkfailed }
    ffi_bind! {
    ///
     cadical_set_opt_checkfrozen(checkfrozen: i32) -> bool => |v|v!=0; as set_opt_checkfrozen }
    ffi_bind! {
    ///
     cadical_set_opt_checkproof(checkproof: i32) -> bool => |v|v!=0; as set_opt_checkproof }
    ffi_bind! {
    ///
     cadical_set_opt_checkwitness(checkwitness: i32) -> bool => |v|v!=0; as set_opt_checkwitness }
    ffi_bind! {
    ///
     cadical_set_opt_chrono(chrono: i32) -> bool => |v|v!=0; as set_opt_chrono }
    ffi_bind! {
    ///
     cadical_set_opt_chronoalways(chronoalways: i32) -> bool => |v|v!=0; as set_opt_chronoalways }
    ffi_bind! {
    ///
     cadical_set_opt_chronolevelim(chronolevelim: i32) -> bool => |v|v!=0; as set_opt_chronolevelim }
    ffi_bind! {
    ///
     cadical_set_opt_chronoreusetrail(chronoreusetrail: i32) -> bool => |v|v!=0; as set_opt_chronoreusetrail }
    ffi_bind! {
    ///
     cadical_set_opt_compact(compact: i32) -> bool => |v|v!=0; as set_opt_compact }
    ffi_bind! {
    ///
     cadical_set_opt_compactint(compactint: i32) -> bool => |v|v!=0; as set_opt_compactint }
    ffi_bind! {
    ///
     cadical_set_opt_compactlim(compactlim: i32) -> bool => |v|v!=0; as set_opt_compactlim }
    ffi_bind! {
    ///
     cadical_set_opt_compactmin(compactmin: i32) -> bool => |v|v!=0; as set_opt_compactmin }
    ffi_bind! {
    ///
     cadical_set_opt_condition(condition: i32) -> bool => |v|v!=0; as set_opt_condition }
    ffi_bind! {
    ///
     cadical_set_opt_conditionint(conditionint: i32) -> bool => |v|v!=0; as set_opt_conditionint }
    ffi_bind! {
    ///
     cadical_set_opt_conditionmaxeff(conditionmaxeff: i32) -> bool => |v|v!=0; as set_opt_conditionmaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_conditionmaxrat(conditionmaxrat: i32) -> bool => |v|v!=0; as set_opt_conditionmaxrat }
    ffi_bind! {
    ///
     cadical_set_opt_conditionmineff(conditionmineff: i32) -> bool => |v|v!=0; as set_opt_conditionmineff }
    ffi_bind! {
    ///
     cadical_set_opt_conditionreleff(conditionreleff: i32) -> bool => |v|v!=0; as set_opt_conditionreleff }
    ffi_bind! {
    ///
     cadical_set_opt_cover(cover: i32) -> bool => |v|v!=0; as set_opt_cover }
    ffi_bind! {
    ///
     cadical_set_opt_covermaxclslim(covermaxclslim: i32) -> bool => |v|v!=0; as set_opt_covermaxclslim }
    ffi_bind! {
    ///
     cadical_set_opt_covermaxeff(covermaxeff: i32) -> bool => |v|v!=0; as set_opt_covermaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_coverminclslim(coverminclslim: i32) -> bool => |v|v!=0; as set_opt_coverminclslim }
    ffi_bind! {
    ///
     cadical_set_opt_covermineff(covermineff: i32) -> bool => |v|v!=0; as set_opt_covermineff }
    ffi_bind! {
    ///
     cadical_set_opt_coverreleff(coverreleff: i32) -> bool => |v|v!=0; as set_opt_coverreleff }
    ffi_bind! {
    ///
     cadical_set_opt_decompose(decompose: i32) -> bool => |v|v!=0; as set_opt_decompose }
    ffi_bind! {
    ///
     cadical_set_opt_decomposerounds(decomposerounds: i32) -> bool => |v|v!=0; as set_opt_decomposerounds }
    ffi_bind! {
    ///
     cadical_set_opt_deduplicate(deduplicate: i32) -> bool => |v|v!=0; as set_opt_deduplicate }
    ffi_bind! {
    ///
     cadical_set_opt_eagersubsume(eagersubsume: i32) -> bool => |v|v!=0; as set_opt_eagersubsume }
    ffi_bind! {
    ///
     cadical_set_opt_eagersubsumelim(eagersubsumelim: i32) -> bool => |v|v!=0; as set_opt_eagersubsumelim }
    ffi_bind! {
    ///
     cadical_set_opt_elim(elim: i32) -> bool => |v|v!=0; as set_opt_elim }
    ffi_bind! {
    ///
     cadical_set_opt_elimands(elimands: i32) -> bool => |v|v!=0; as set_opt_elimands }
    ffi_bind! {
    ///
     cadical_set_opt_elimaxeff(elimaxeff: i32) -> bool => |v|v!=0; as set_opt_elimaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_elimbackward(elimbackward: i32) -> bool => |v|v!=0; as set_opt_elimbackward }
    ffi_bind! {
    ///
     cadical_set_opt_elimboundmax(elimboundmax: i32) -> bool => |v|v!=0; as set_opt_elimboundmax }
    ffi_bind! {
    ///
     cadical_set_opt_elimboundmin(elimboundmin: i32) -> bool => |v|v!=0; as set_opt_elimboundmin }
    ffi_bind! {
    ///
     cadical_set_opt_elimclslim(elimclslim: i32) -> bool => |v|v!=0; as set_opt_elimclslim }
    ffi_bind! {
    ///
     cadical_set_opt_elimequivs(elimequivs: i32) -> bool => |v|v!=0; as set_opt_elimequivs }
    ffi_bind! {
    ///
     cadical_set_opt_elimineff(elimineff: i32) -> bool => |v|v!=0; as set_opt_elimineff }
    ffi_bind! {
    ///
     cadical_set_opt_elimint(elimint: i32) -> bool => |v|v!=0; as set_opt_elimint }
    ffi_bind! {
    ///
     cadical_set_opt_elimites(elimites: i32) -> bool => |v|v!=0; as set_opt_elimites }
    ffi_bind! {
    ///
     cadical_set_opt_elimlimited(elimlimited: i32) -> bool => |v|v!=0; as set_opt_elimlimited }
    ffi_bind! {
    ///
     cadical_set_opt_elimocclim(elimocclim: i32) -> bool => |v|v!=0; as set_opt_elimocclim }
    ffi_bind! {
    ///
     cadical_set_opt_elimprod(elimprod: i32) -> bool => |v|v!=0; as set_opt_elimprod }
    ffi_bind! {
    ///
     cadical_set_opt_elimreleff(elimreleff: i32) -> bool => |v|v!=0; as set_opt_elimreleff }
    ffi_bind! {
    ///
     cadical_set_opt_elimrounds(elimrounds: i32) -> bool => |v|v!=0; as set_opt_elimrounds }
    ffi_bind! {
    ///
     cadical_set_opt_elimsubst(elimsubst: i32) -> bool => |v|v!=0; as set_opt_elimsubst }
    ffi_bind! {
    ///
     cadical_set_opt_elimsum(elimsum: i32) -> bool => |v|v!=0; as set_opt_elimsum }
    ffi_bind! {
    ///
     cadical_set_opt_elimxorlim(elimxorlim: i32) -> bool => |v|v!=0; as set_opt_elimxorlim }
    ffi_bind! {
    ///
     cadical_set_opt_elimxors(elimxors: i32) -> bool => |v|v!=0; as set_opt_elimxors }
    ffi_bind! {
    ///
     cadical_set_opt_emagluefast(emagluefast: i32) -> bool => |v|v!=0; as set_opt_emagluefast }
    ffi_bind! {
    ///
     cadical_set_opt_emaglueslow(emaglueslow: i32) -> bool => |v|v!=0; as set_opt_emaglueslow }
    ffi_bind! {
    ///
     cadical_set_opt_emajump(emajump: i32) -> bool => |v|v!=0; as set_opt_emajump }
    ffi_bind! {
    ///
     cadical_set_opt_emalevel(emalevel: i32) -> bool => |v|v!=0; as set_opt_emalevel }
    ffi_bind! {
    ///
     cadical_set_opt_emasize(emasize: i32) -> bool => |v|v!=0; as set_opt_emasize }
    ffi_bind! {
    ///
     cadical_set_opt_ematrailfast(ematrailfast: i32) -> bool => |v|v!=0; as set_opt_ematrailfast }
    ffi_bind! {
    ///
     cadical_set_opt_ematrailslow(ematrailslow: i32) -> bool => |v|v!=0; as set_opt_ematrailslow }
    ffi_bind! {
    ///
     cadical_set_opt_exteagerreasons(exteagerreasons: i32) -> bool => |v|v!=0; as set_opt_exteagerreasons }
    ffi_bind! {
    ///
     cadical_set_opt_exteagerrecalc(exteagerrecalc: i32) -> bool => |v|v!=0; as set_opt_exteagerrecalc }
    ffi_bind! {
    ///
     cadical_set_opt_externallrat(externallrat: i32) -> bool => |v|v!=0; as set_opt_externallrat }
    ffi_bind! {
    ///
     cadical_set_opt_flush(flush: i32) -> bool => |v|v!=0; as set_opt_flush }
    ffi_bind! {
    ///
     cadical_set_opt_flushfactor(flushfactor: i32) -> bool => |v|v!=0; as set_opt_flushfactor }
    ffi_bind! {
    ///
     cadical_set_opt_flushint(flushint: i32) -> bool => |v|v!=0; as set_opt_flushint }
    ffi_bind! {
    ///
     cadical_set_opt_forcephase(forcephase: i32) -> bool => |v|v!=0; as set_opt_forcephase }
    ffi_bind! {
    ///
     cadical_set_opt_frat(frat: i32) -> bool => |v|v!=0; as set_opt_frat }
    ffi_bind! {
    ///
     cadical_set_opt_idrup(idrup: i32) -> bool => |v|v!=0; as set_opt_idrup }
    ffi_bind! {
    ///
     cadical_set_opt_ilb(ilb: i32) -> bool => |v|v!=0; as set_opt_ilb }
    ffi_bind! {
    ///
     cadical_set_opt_ilbassumptions(ilbassumptions: i32) -> bool => |v|v!=0; as set_opt_ilbassumptions }
    ffi_bind! {
    ///
     cadical_set_opt_inprocessing(inprocessing: i32) -> bool => |v|v!=0; as set_opt_inprocessing }
    ffi_bind! {
    ///
     cadical_set_opt_instantiate(instantiate: i32) -> bool => |v|v!=0; as set_opt_instantiate }
    ffi_bind! {
    ///
     cadical_set_opt_instantiateclslim(instantiateclslim: i32) -> bool => |v|v!=0; as set_opt_instantiateclslim }
    ffi_bind! {
    ///
     cadical_set_opt_instantiateocclim(instantiateocclim: i32) -> bool => |v|v!=0; as set_opt_instantiateocclim }
    ffi_bind! {
    ///
     cadical_set_opt_instantiateonce(instantiateonce: i32) -> bool => |v|v!=0; as set_opt_instantiateonce }
    ffi_bind! {
    ///
     cadical_set_opt_lidrup(lidrup: i32) -> bool => |v|v!=0; as set_opt_lidrup }
    ffi_bind! {
    ///
     cadical_set_opt_log(log: i32) -> bool => |v|v!=0; as set_opt_log }
    ffi_bind! {
    ///
     cadical_set_opt_logsort(logsort: i32) -> bool => |v|v!=0; as set_opt_logsort }
    ffi_bind! {
    ///
     cadical_set_opt_lrat(lrat: i32) -> bool => |v|v!=0; as set_opt_lrat }
    ffi_bind! {
    ///
     cadical_set_opt_lucky(lucky: i32) -> bool => |v|v!=0; as set_opt_lucky }
    ffi_bind! {
    ///
     cadical_set_opt_minimize(minimize: i32) -> bool => |v|v!=0; as set_opt_minimize }
    ffi_bind! {
    ///
     cadical_set_opt_minimizedepth(minimizedepth: i32) -> bool => |v|v!=0; as set_opt_minimizedepth }
    ffi_bind! {
    ///
     cadical_set_opt_otfs(otfs: i32) -> bool => |v|v!=0; as set_opt_otfs }
    ffi_bind! {
    ///
     cadical_set_opt_phase(phase: i32) -> bool => |v|v!=0; as set_opt_phase }
    ffi_bind! {
    ///
     cadical_set_opt_probe(probe: i32) -> bool => |v|v!=0; as set_opt_probe }
    ffi_bind! {
    ///
     cadical_set_opt_probehbr(probehbr: i32) -> bool => |v|v!=0; as set_opt_probehbr }
    ffi_bind! {
    ///
     cadical_set_opt_probeint(probeint: i32) -> bool => |v|v!=0; as set_opt_probeint }
    ffi_bind! {
    ///
     cadical_set_opt_probemaxeff(probemaxeff: i32) -> bool => |v|v!=0; as set_opt_probemaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_probemineff(probemineff: i32) -> bool => |v|v!=0; as set_opt_probemineff }
    ffi_bind! {
    ///
     cadical_set_opt_probereleff(probereleff: i32) -> bool => |v|v!=0; as set_opt_probereleff }
    ffi_bind! {
    ///
     cadical_set_opt_proberounds(proberounds: i32) -> bool => |v|v!=0; as set_opt_proberounds }
    ffi_bind! {
    ///
     cadical_set_opt_profile(profile: i32) -> bool => |v|v!=0; as set_opt_profile }
    ffi_bind! {
    ///
     cadical_set_opt_quiet(quiet: i32) -> bool => |v|v!=0; as set_opt_quiet }
    ffi_bind! {
    ///
     cadical_set_opt_radixsortlim(radixsortlim: i32) -> bool => |v|v!=0; as set_opt_radixsortlim }
    ffi_bind! {
    ///
     cadical_set_opt_realtime(realtime: i32) -> bool => |v|v!=0; as set_opt_realtime }
    ffi_bind! {
    ///
     cadical_set_opt_reduce(reduce: i32) -> bool => |v|v!=0; as set_opt_reduce }
    ffi_bind! {
    ///
     cadical_set_opt_reduceint(reduceint: i32) -> bool => |v|v!=0; as set_opt_reduceint }
    ffi_bind! {
    ///
     cadical_set_opt_reducetarget(reducetarget: i32) -> bool => |v|v!=0; as set_opt_reducetarget }
    ffi_bind! {
    ///
     cadical_set_opt_reducetier1glue(reducetier1glue: i32) -> bool => |v|v!=0; as set_opt_reducetier1glue }
    ffi_bind! {
    ///
     cadical_set_opt_reducetier2glue(reducetier2glue: i32) -> bool => |v|v!=0; as set_opt_reducetier2glue }
    ffi_bind! {
    ///
     cadical_set_opt_reluctant(reluctant: i32) -> bool => |v|v!=0; as set_opt_reluctant }
    ffi_bind! {
    ///
     cadical_set_opt_reluctantmax(reluctantmax: i32) -> bool => |v|v!=0; as set_opt_reluctantmax }
    ffi_bind! {
    ///
     cadical_set_opt_rephase(rephase: i32) -> bool => |v|v!=0; as set_opt_rephase }
    ffi_bind! {
    ///
     cadical_set_opt_rephaseint(rephaseint: i32) -> bool => |v|v!=0; as set_opt_rephaseint }
    ffi_bind! {
    ///
     cadical_set_opt_report(report: i32) -> bool => |v|v!=0; as set_opt_report }
    ffi_bind! {
    ///
     cadical_set_opt_reportall(reportall: i32) -> bool => |v|v!=0; as set_opt_reportall }
    ffi_bind! {
    ///
     cadical_set_opt_reportsolve(reportsolve: i32) -> bool => |v|v!=0; as set_opt_reportsolve }
    ffi_bind! {
    ///
     cadical_set_opt_restart(restart: i32) -> bool => |v|v!=0; as set_opt_restart }
    ffi_bind! {
    ///
     cadical_set_opt_restartint(restartint: i32) -> bool => |v|v!=0; as set_opt_restartint }
    ffi_bind! {
    ///
     cadical_set_opt_restartmargin(restartmargin: i32) -> bool => |v|v!=0; as set_opt_restartmargin }
    ffi_bind! {
    ///
     cadical_set_opt_restartreusetrail(restartreusetrail: i32) -> bool => |v|v!=0; as set_opt_restartreusetrail }
    ffi_bind! {
    ///
     cadical_set_opt_restoreall(restoreall: i32) -> bool => |v|v!=0; as set_opt_restoreall }
    ffi_bind! {
    ///
     cadical_set_opt_restoreflush(restoreflush: i32) -> bool => |v|v!=0; as set_opt_restoreflush }
    ffi_bind! {
    ///
     cadical_set_opt_reverse(reverse: i32) -> bool => |v|v!=0; as set_opt_reverse }
    ffi_bind! {
    ///
     cadical_set_opt_score(score: i32) -> bool => |v|v!=0; as set_opt_score }
    ffi_bind! {
    ///
     cadical_set_opt_scorefactor(scorefactor: i32) -> bool => |v|v!=0; as set_opt_scorefactor }
    ffi_bind! {
    ///
     cadical_set_opt_seed(seed: i32) -> bool => |v|v!=0; as set_opt_seed }
    ffi_bind! {
    ///
     cadical_set_opt_shrink(shrink: i32) -> bool => |v|v!=0; as set_opt_shrink }
    ffi_bind! {
    ///
     cadical_set_opt_shrinkreap(shrinkreap: i32) -> bool => |v|v!=0; as set_opt_shrinkreap }
    ffi_bind! {
    ///
     cadical_set_opt_shuffle(shuffle: i32) -> bool => |v|v!=0; as set_opt_shuffle }
    ffi_bind! {
    ///
     cadical_set_opt_shufflequeue(shufflequeue: i32) -> bool => |v|v!=0; as set_opt_shufflequeue }
    ffi_bind! {
    ///
     cadical_set_opt_shufflerandom(shufflerandom: i32) -> bool => |v|v!=0; as set_opt_shufflerandom }
    ffi_bind! {
    ///
     cadical_set_opt_shufflescores(shufflescores: i32) -> bool => |v|v!=0; as set_opt_shufflescores }
    ffi_bind! {
    ///
     cadical_set_opt_stabilize(stabilize: i32) -> bool => |v|v!=0; as set_opt_stabilize }
    ffi_bind! {
    ///
     cadical_set_opt_stabilizefactor(stabilizefactor: i32) -> bool => |v|v!=0; as set_opt_stabilizefactor }
    ffi_bind! {
    ///
     cadical_set_opt_stabilizeint(stabilizeint: i32) -> bool => |v|v!=0; as set_opt_stabilizeint }
    ffi_bind! {
    ///
     cadical_set_opt_stabilizemaxint(stabilizemaxint: i32) -> bool => |v|v!=0; as set_opt_stabilizemaxint }
    ffi_bind! {
    ///
     cadical_set_opt_stabilizeonly(stabilizeonly: i32) -> bool => |v|v!=0; as set_opt_stabilizeonly }
    ffi_bind! {
    ///
     cadical_set_opt_stats(stats: i32) -> bool => |v|v!=0; as set_opt_stats }
    ffi_bind! {
    ///
     cadical_set_opt_subsume(subsume: i32) -> bool => |v|v!=0; as set_opt_subsume }
    ffi_bind! {
    ///
     cadical_set_opt_subsumebinlim(subsumebinlim: i32) -> bool => |v|v!=0; as set_opt_subsumebinlim }
    ffi_bind! {
    ///
     cadical_set_opt_subsumeclslim(subsumeclslim: i32) -> bool => |v|v!=0; as set_opt_subsumeclslim }
    ffi_bind! {
    ///
     cadical_set_opt_subsumeint(subsumeint: i32) -> bool => |v|v!=0; as set_opt_subsumeint }
    ffi_bind! {
    ///
     cadical_set_opt_subsumelimited(subsumelimited: i32) -> bool => |v|v!=0; as set_opt_subsumelimited }
    ffi_bind! {
    ///
     cadical_set_opt_subsumemaxeff(subsumemaxeff: i32) -> bool => |v|v!=0; as set_opt_subsumemaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_subsumemineff(subsumemineff: i32) -> bool => |v|v!=0; as set_opt_subsumemineff }
    ffi_bind! {
    ///
     cadical_set_opt_subsumeocclim(subsumeocclim: i32) -> bool => |v|v!=0; as set_opt_subsumeocclim }
    ffi_bind! {
    ///
     cadical_set_opt_subsumereleff(subsumereleff: i32) -> bool => |v|v!=0; as set_opt_subsumereleff }
    ffi_bind! {
    ///
     cadical_set_opt_subsumestr(subsumestr: i32) -> bool => |v|v!=0; as set_opt_subsumestr }
    ffi_bind! {
    ///
     cadical_set_opt_target(target: i32) -> bool => |v|v!=0; as set_opt_target }
    ffi_bind! {
    ///
     cadical_set_opt_terminateint(terminateint: i32) -> bool => |v|v!=0; as set_opt_terminateint }
    ffi_bind! {
    ///
     cadical_set_opt_ternary(ternary: i32) -> bool => |v|v!=0; as set_opt_ternary }
    ffi_bind! {
    ///
     cadical_set_opt_ternarymaxadd(ternarymaxadd: i32) -> bool => |v|v!=0; as set_opt_ternarymaxadd }
    ffi_bind! {
    ///
     cadical_set_opt_ternarymaxeff(ternarymaxeff: i32) -> bool => |v|v!=0; as set_opt_ternarymaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_ternarymineff(ternarymineff: i32) -> bool => |v|v!=0; as set_opt_ternarymineff }
    ffi_bind! {
    ///
     cadical_set_opt_ternaryocclim(ternaryocclim: i32) -> bool => |v|v!=0; as set_opt_ternaryocclim }
    ffi_bind! {
    ///
     cadical_set_opt_ternaryreleff(ternaryreleff: i32) -> bool => |v|v!=0; as set_opt_ternaryreleff }
    ffi_bind! {
    ///
     cadical_set_opt_ternaryrounds(ternaryrounds: i32) -> bool => |v|v!=0; as set_opt_ternaryrounds }
    ffi_bind! {
    ///
     cadical_set_opt_transred(transred: i32) -> bool => |v|v!=0; as set_opt_transred }
    ffi_bind! {
    ///
     cadical_set_opt_transredmaxeff(transredmaxeff: i32) -> bool => |v|v!=0; as set_opt_transredmaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_transredmineff(transredmineff: i32) -> bool => |v|v!=0; as set_opt_transredmineff }
    ffi_bind! {
    ///
     cadical_set_opt_transredreleff(transredreleff: i32) -> bool => |v|v!=0; as set_opt_transredreleff }
    ffi_bind! {
    ///
     cadical_set_opt_verbose(verbose: i32) -> bool => |v|v!=0; as set_opt_verbose }
    ffi_bind! {
    ///
     cadical_set_opt_veripb(veripb: i32) -> bool => |v|v!=0; as set_opt_veripb }
    ffi_bind! {
    ///
     cadical_set_opt_vivify(vivify: i32) -> bool => |v|v!=0; as set_opt_vivify }
    ffi_bind! {
    ///
     cadical_set_opt_vivifyinst(vivifyinst: i32) -> bool => |v|v!=0; as set_opt_vivifyinst }
    ffi_bind! {
    ///
     cadical_set_opt_vivifymaxeff(vivifymaxeff: i32) -> bool => |v|v!=0; as set_opt_vivifymaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_vivifymineff(vivifymineff: i32) -> bool => |v|v!=0; as set_opt_vivifymineff }
    ffi_bind! {
    ///
     cadical_set_opt_vivifyonce(vivifyonce: i32) -> bool => |v|v!=0; as set_opt_vivifyonce }
    ffi_bind! {
    ///
     cadical_set_opt_vivifyredeff(vivifyredeff: i32) -> bool => |v|v!=0; as set_opt_vivifyredeff }
    ffi_bind! {
    ///
     cadical_set_opt_vivifyreleff(vivifyreleff: i32) -> bool => |v|v!=0; as set_opt_vivifyreleff }
    ffi_bind! {
    ///
     cadical_set_opt_walk(walk: i32) -> bool => |v|v!=0; as set_opt_walk }
    ffi_bind! {
    ///
     cadical_set_opt_walkmaxeff(walkmaxeff: i32) -> bool => |v|v!=0; as set_opt_walkmaxeff }
    ffi_bind! {
    ///
     cadical_set_opt_walkmineff(walkmineff: i32) -> bool => |v|v!=0; as set_opt_walkmineff }
    ffi_bind! {
    ///
     cadical_set_opt_walknonstable(walknonstable: i32) -> bool => |v|v!=0; as set_opt_walknonstable }
    ffi_bind! {
    ///
     cadical_set_opt_walkredundant(walkredundant: i32) -> bool => |v|v!=0; as set_opt_walkredundant }
    ffi_bind! {
    ///
     cadical_set_opt_walkreleff(walkreleff: i32) -> bool => |v|v!=0; as set_opt_walkreleff }
}

impl SatSolver for CaDiCaLSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError> {
        CaDiCaLSolver::add_clause(self, clause)
    }

    fn solve_sat(&mut self) -> Result<RawStatus, SolverError> {
        CaDiCaLSolver::solve(self)
    }

    fn model(&mut self) -> Result<Vec<i32>, SolverError> {
        let vars = self.vars()?;
        let mut model = vec![];
        for lit in 1..=vars {
            if self.val(lit)? > 0 {
                model.push(lit);
            }
        }
        Ok(model)
    }
}
impl Drop for CaDiCaLSolver {
    fn drop(&mut self) {
        unsafe {
            binding::cadical_destroy(self.inner.as_ptr());
        }
    }
}
