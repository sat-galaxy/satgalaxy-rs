# rssat


[<img alt="github" src="https://img.shields.io/badge/github-francisol/rssat?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/francisol/rssat)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rssat.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rssat)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rssat?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rssat)

**rssat** is a Rust library that provides Rust bindings for multiple popular SAT solvers. Currently supported solvers include:

- [MiniSat](https://github.com/niklasso/minisat) (2.2.0)
- [Glucose](https://github.com/audemard/glucose) (4.2.1)
- [CaDiCaL](https://github.com/arminbiere/cadical) (2.1.3)

We thank the contributors of these excellent projects.
## Features

- Unified Rust interface for different SAT solvers
- Support for adding clauses
- Solving SAT problems and returning results
- Access to native bindings for advanced functionality
- Support reading formulas from files


## Build Requirements
To build RSsat, you need the following tools and libraries:

- C++ compiler (e.g., GCC, Clang)
- CMake (>3.10)
- patch command
- Other standard build tools (make, etc.)

## Installation

```toml
[dependencies]
rssat = "0.1.5"
```

## Usage Example
Here's a simple example using the CaDiCaL solver:
```rust
use rssat::solver::{CaDiCaLSolver, Status,Solver};

fn main() {
    let mut solver = CaDiCaLSolver::new();
    
    solver.add_clause(&vec![1, 2]);
    solver.add_clause(&vec![-1, -2]);
    solver.add_clause(&vec![3]);
    
    
    match solver.solve() {
        Status::SATISFIABLE(vec) => {
            println!("Satisfiable solution: {:?}", vec);
        },
        Status::UNSATISFIABLE => {
            println!("Unsatisfiable");
        },
        Status::UNKNOWN => {
            println!("Unknown");
        },
    }
}
```
## Native Bindings
For advanced usage, you can access the native bindings of each solver. This allows you to use solver-specific features that are not part of the unified interface. 

## Future Work
- Improve documentation to enhance user experience


## Contributing
Issue reports and pull requests are welcome!
## License
MIT License


