pub(self) mod base;
#[cfg(feature = "cadical")]
pub mod cadical;
#[cfg(feature = "cadical")]
pub use cadical::CaDiCaLSolver;
#[cfg(feature = "glucose")]
pub(self) mod glucose;
#[cfg(feature = "glucose")]
pub use glucose::GlucoseSolver;
#[cfg(feature = "minisat")]
pub mod minisat;
#[cfg(feature = "minisat")]
pub  use  minisat::MinisatSolver;

pub use base::Solver;
pub use base::Status;
#[macro_export]
macro_rules! create_solver {
    (cadical) => {
        $crate::cadical::CaDiCaLSolver::new()
    };
    (minisat) => {
        $crate::minisat::MiniSatSolver::new()
    };
}
