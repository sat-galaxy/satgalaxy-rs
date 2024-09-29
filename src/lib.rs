
pub mod solver;

#[cfg(test)]
mod tests {
    use solver::{Solver, Status};

    use super::*;

    #[test]
    #[cfg(feature = "cadical")]
    fn cadical() {
        let mut solver= solver::cadical::CaDiCaLSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve() {
            Status::SATISFIABLE(vec) => {
                assert_eq!(1,0);
            },
            Status::UNSATISFIABLE => {
                assert_eq!(true,true)
            },
            Status::UNKNOWN => {
                assert_eq!(10,0);

            },
        }
    }
    #[test]
    #[cfg(feature = "minisat")]
    fn minisat() {
        let mut solver= solver::minisat::MinisatSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve() {
            Status::SATISFIABLE(vec) => {
                assert_eq!(1,0);
            },
            Status::UNSATISFIABLE => {
                assert_eq!(true,true)
            },
            Status::UNKNOWN => {
                assert_eq!(10,0);

            },
        }
    }
    #[test]
    #[cfg(feature = "glucose")]
    fn glucose() {
        let mut solver= solver::glucose::GlucoseSolver::new();
        solver.add_clause(&vec![1]);
        solver.add_clause(&vec![-1]);
        match solver.solve() {
            Status::SATISFIABLE(vec) => {
                assert_eq!(1,0);
            },
            Status::UNSATISFIABLE => {
                assert_eq!(true,true)
            },
            Status::UNKNOWN => {
                assert_eq!(10,0);

            },
        }
    }
}
