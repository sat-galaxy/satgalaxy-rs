//! # rssat
//! 
//! [<img alt="github" src="https://img.shields.io/badge/github-francisol/rssat?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/francisol/rssat)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/rssat.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rssat)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rssat?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rssat)
//! 
//! **rssat** is a Rust library that provides Rust bindings for multiple popular SAT solvers. Currently supported solvers include:
//! 
//! - [MiniSat](https://github.com/niklasso/minisat) (2.2.0)
//! - [Glucose](https://github.com/audemard/glucose) (4.2.1)
//! - [CaDiCaL](https://github.com/arminbiere/cadical) (2.1.3)
//! 
//! We thank the contributors of these excellent projects.
//! ## Features
//! 
//! - Unified Rust interface for different SAT solvers
//! - Support for adding clauses
//! - Solving SAT problems and returning results
//! - Access to native bindings for advanced functionality
//! 
//! ## Build Requirements
//! To build RSsat, you need the following tools and libraries:
//!
//!  - C++ compiler (e.g., GCC, Clang)
//!  - CMake (>3.10)
//!  - patch command
//!  - Other standard build tools (make, etc.)
//! 
//! 
//! ## Installation
//! Currently, RSsat is not published on crates.io. We plan to publish it in the future. Until then, you can use it via Git repository:
//! ```toml
//! [dependencies]
//! rssat = "0.1.5"
//! ```
//! 
//! ## Usage Example
//! Here's a simple example using the CaDiCaL solver:
//! ```rust
//! use rssat::solver::{CaDiCaLSolver, Status,Solver};
//! use rssat::parser::{parse_dimacs_cnf,read_dimacs_from_file};
//! fn main() {
//!     let mut solver = CaDiCaLSolver::new();
//!     let cnf_formula = parse_dimacs_cnf("c This is a comment
//!        p cnf 3 2
//!        1 -3 0
//!        ",false);
//!    // or call read_dimacs_from_file("path to file",false);
//!     
//!     solver.add_clause(&vec![1, 2]);
//!     solver.add_clause(&vec![-1, -2]);
//!     solver.add_clause(&vec![3]);
//!     
//!     
//!     match solver.solve() {
//!         Status::SATISFIABLE(vec) => {
//!             println!("Satisfiable solution: {:?}", vec);
//!         },
//!         Status::UNSATISFIABLE => {
//!             println!("Unsatisfiable");
//!         },
//!         Status::UNKNOWN => {
//!             println!("Unknown");
//!         },
//!     }
//! }
//! ```
//! ## Native Bindings
//! For advanced usage, you can access the native bindings of each solver. This allows you to use solver-specific features that are not part of the unified interface. 
//! 
//! ## Future Work
//! - Submit the package to crates.io
//! - Improve documentation to enhance user experience
//! - Support reading formulas from files
//! 
//! ## Contributing
//! Issue reports and pull requests are welcome!
//! ## License
//! MIT License
//! 
//! 
pub mod solver;
pub  mod  errors;
#[cfg(feature = "parser")]
pub mod parser;
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
            Status::SATISFIABLE(_vec) => {
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
        let mut solver= solver::MinisatSolver::new();
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
    #[test]
    #[cfg(feature="dimacs")]
    fn dimacs(){
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 3 2
        1 -3 0
        ";
            match parse_dimacs_cnf(dimacs_content,false) {
                Ok(cnf) => {
                    assert_eq!(cnf.num_vars,3);
                    assert_eq!(cnf.num_clauses,1);
                },
                Err(e) => assert_eq!("result","should be ok"),
            }
      
    }
    #[test]
    #[cfg(feature="dimacs")]
    fn dimacs_strict(){
        use parser::parse_dimacs_cnf;

        let dimacs_content = "c This is a comment
        p cnf 2 2
        1 -3 0
        ";
            match parse_dimacs_cnf(dimacs_content,true) {
                Ok(cnf) => {
                    assert_eq!("result","should be error")
                },
                Err(e) => assert!(true),
            }
      
    }

}
