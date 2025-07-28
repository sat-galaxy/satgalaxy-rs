#![doc = include_str!("../README.md")]
pub mod errors;
#[cfg(feature = "parser")]
pub mod parser;
pub mod solver;
#[cfg(test)]
mod tests {
    use solver::{SatSolver, SatStatus};

    use super::*;

    #[test]
    #[cfg(feature = "cadical")]
    fn cadical() {
        let mut solver = solver::cadical::CaDiCaLSolver::new();
        solver.push_clause(&vec![1]).unwrap();
        solver.push_clause(&vec![-1]).unwrap();
        match solver.solve_model().unwrap() {
            SatStatus::Satisfiable(_vec) => {
                assert_eq!(1, 0);
            }
            SatStatus::Unsatisfiable => {
                assert_eq!(true, true)
            }
            SatStatus::Unknown => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "minisat")]
    fn minisat() {
        let mut solver = solver::MinisatSolver::new();
        solver.push_clause(&[1]).unwrap();
        solver.push_clause(&[-1]).unwrap();
        match solver.solve_model().unwrap() {
            SatStatus::Satisfiable(_vec) => {
                assert_eq!(1, 0);
            }
            SatStatus::Unsatisfiable => {
                assert_eq!(true, true)
            }
            SatStatus::Unknown => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "glucose")]
    fn glucose() {
        let mut solver = solver::glucose::GlucoseSolver::new();
        solver.push_clause(&[1]).unwrap();
        solver.push_clause(&[-1]).unwrap();
        match solver.solve_model().unwrap() {
            SatStatus::Satisfiable(_vec) => {
                assert_eq!(1, 0);
            }
            SatStatus::Unsatisfiable => {
                assert_eq!(true, true)
            }
            SatStatus::Unknown => {
                assert_eq!(10, 0);
            }
        }
    }
    #[test]
    #[cfg(feature = "parser")]
    fn dimacs() {
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 3 2
        1 -3 0
        ";
        let mut cnf=Vec::new();
        match parse_dimacs_cnf(dimacs_content, false,&mut cnf) {
            Ok(_) => {
                assert_eq!(cnf.len(), 1);
            }
            Err(_e) => assert_eq!("result", "should be ok"),
        }
    }
    #[test]
    #[cfg(feature = "parser")]
    fn dimacs_strict() {
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 2 2
        1 -3 0
        ";
        let mut cnf=Vec::new();
        match parse_dimacs_cnf(dimacs_content, true,&mut cnf) {
            Ok(_) => {
                assert_eq!("result", "should be error")
            }
            Err(_e) => assert!(true),
        }
    }
}
