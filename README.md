# rssat
**rssat** is a Rust library that provides Rust bindings for multiple popular SAT solvers. Currently supported solvers include:

- [MiniSat](https://github.com/niklasso/minisat) (2.2.0)
- [Glucose](https://github.com/audemard/glucose) (4.2.1)
- [CaDiCaL](https://github.com/arminbiere/cadical) (2.0.0)

We thank the contributors of these excellent projects.
## Features

- Unified Rust interface for different SAT solvers
- Support for adding clauses
- Solving SAT problems and returning results
- Access to native bindings for advanced functionality


## Build Requirements
To build RSsat, you need the following tools and libraries:

- C++ compiler (e.g., GCC, Clang)
- CMake
- patch command
- Other standard build tools (make, etc.)

## Installation
Currently, RSsat is not published on crates.io. We plan to publish it in the future. Until then, you can use it via Git repository:
```toml
[dependencies]
rssat = { git = "https://github.com/francisol/rssat.git" }
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
- Submit the package to crates.io
- Improve documentation to enhance user experience
- Support reading formulas from files

## Contributing
Issue reports and pull requests are welcome!
## License
MIT License


