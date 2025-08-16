#[cfg(feature = "cadical")]
pub mod cadical;
#[cfg(feature = "cadical")]
pub use cadical::CaDiCaLSolver;
#[cfg(feature = "glucose")]
pub mod glucose;
#[cfg(feature = "glucose")]
pub use glucose::GlucoseSolver;
#[cfg(feature = "minisat")]
pub mod minisat;
#[cfg(feature = "minisat")]
pub use minisat::MinisatSolver;
#[cfg(feature = "picosat")]
pub mod picosat;
#[cfg(feature = "picosat")]
pub use picosat::PicoSATSolver;

use crate::errors::SolverError;

#[macro_export]
macro_rules! create_solver {
    (cadical) => {
        $crate::cadical::CaDiCaLSolver::new()
    };
    (minisat) => {
        $crate::minisat::MiniSatSolver::new()
    };
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawStatus {
    Satisfiable,
    Unsatisfiable,
    Unknown,
}

impl Default for RawStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<i32> for RawStatus {
    fn from(value: i32) -> Self {
        match value {
            10 => Self::Satisfiable,
            20 => Self::Unsatisfiable,
            _ => Self::Unknown,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SatStatus {
    Satisfiable(Vec<i32>),
    Unsatisfiable,
    Unknown,
}

impl Default for SatStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

pub trait SatSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError>;

    fn solve_model(&mut self) -> Result<SatStatus, SolverError> {
        let status = self.solve_sat()?;
        return match status {
            RawStatus::Satisfiable => self.model().map(SatStatus::Satisfiable),
            RawStatus::Unsatisfiable => Ok(SatStatus::Unsatisfiable),
            RawStatus::Unknown => Ok(SatStatus::Unknown),
        };
    }
    fn solve_sat(&mut self) -> Result<RawStatus, SolverError>;
    fn model(&mut self) -> Result<Vec<i32>, SolverError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusStatus {
    Satisfiable,
    Unsatisfiable(Vec<usize>),
    Unknown,
}

impl Default for MusStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

pub trait MusSolver {
    fn push_clause(&mut self, clause: &[i32]) -> Result<(), SolverError>;

    fn solve_mus(&mut self) -> Result<MusStatus, SolverError>;
}
