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

#[macro_export]
macro_rules! create_solver {
    (cadical) => {
        $crate::cadical::CaDiCaLSolver::new()
    };
    (minisat) => {
        $crate::minisat::MiniSatSolver::new()
    };
}

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
pub enum Status {
    Satisfiable(Vec<i32>),
    Unsatisfiable,
    Unknown,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

pub trait SatSolver {
    fn add_clause(& self, clause: &[i32]);
    fn solve_model(& self) -> Status;
}
