# satgalaxy-rs 🌠

**Rust FFI bindings for `satgalaxy-core` – bringing high-performance SAT solving to Rust!**

[![crates.io](https://img.shields.io/crates/v/satgalaxy)](https://crates.io/crates/satgalaxy)
[![docs.rs](https://docs.rs/satgalaxy/badge.svg)](https://docs.rs/satgalaxy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)](https://github.com/your-username/satgalaxy-rs/actions) 
---
### Platform & Build Compatibility

![Linux](https://img.shields.io/badge/OS-Linux-informational?logo=linux&logoColor=white)
![macOS](https://img.shields.io/badge/OS-macOS-informational?logo=apple&logoColor=white)
![Windows](https://img.shields.io/badge/OS-Windows-informational?logo=windows&logoColor=white)
![Build System: CMake](https://img.shields.io/badge/Build%20System-CMake-blue?logo=cmake&logoColor=white)

---
## 🚧 Under Development

**Please note**: `satgalaxy-rs` **is currently under active development.

---

## 🌟 Overview

`satgalaxy-rs` is a Rust Foreign Function Interface (FFI) library that provides safe and idiomatic Rust bindings to the  `satgalaxy-core` C library. This means you can now leverage the power of high-performance SAT solvers like Minisat and Glucose directly from your Rust applications, without worrying about low-level C interoperability details.

By using `satgalaxy-rs`, you get:

- **Access to Battle-Tested Solvers**: Benefit from the speed and robustness of Minisat and Glucose, widely used in research and industry.
- **Safe Rust API**: Interact with the C solvers through a Rust-idiomatic and memory-safe interface.
- **Cross-Platform Compatibility**: Inherit the multi-operating system support provided by satgalaxy-core (Linux, macOS, Windows).
- **Simplified Integration**: No need to manually compile C code; satgalaxy-rs handles the satgalaxy-core dependency during its build process.
## ✨ Supported Solvers
Currently, the following SAT solvers are supported：

- Minisat([GitHub](https://github.com/niklasso/minisat))
- Glucose([4.2.1](http://www.labri.fr/perso/lsimon/glucose/))
- CaDiCaL([rel-2.1.3](https://github.com/arminbiere/cadical))
- PicoSAT([960](https://fmv.jku.at/picosat/))

Currently, the following MUS solver are supported：
- PicoSAT([960](https://fmv.jku.at/picosat/))

## 🚀 Getting Started

### Prerequisites

To use `satgalaxy-rs`, you'll need:

- **Rust Toolchain**: Install Rust via rustup (https://rustup.rs/).

- **C/C++ Compiler**: A C/C++ compiler (like GCC, Clang, MSVC) compatible with your system is required to compile satgalaxy-core.

### Installation

Add `satgalaxy-rs` to your Cargo.toml dependencies:
```toml
[dependencies]
satgalaxy = { version = "0.2.0", features = ["minisat"] }
```
### Basic Usage

Here's a quick example showing how to solve a SAT problem using `satgalaxy-rs` with the Minisat backend:
```rust
use satgalaxy::{MinisatSolver, SatStatus,SatSolver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Minisat solver instance
    let mut solver = MinisatSolver::new();


    // Add clauses. A clause is a disjunction of literals.
    // Example: (var1 OR NOT var2)
    solver.push_clause(&[1, -2])?;

    // Example: (var2 OR var3)
    solver.push_clause(&[2, 3])?;

    // Solve the SAT problem
    println!("Attempting to solve...");
    match solver.solve_model().unwrap(){ // Pass an empty slice for assumptions
        SatStatus::Satisfiable(model) => {
            println!("SATISFIABLE! 🎉");
            // Retrieve and print the model (variable assignments)
            println!("Model: {:?} ",model);
        },
        SatStatus::Unsatisfiable => {
            println!("UNSATISFIABLE! 😞");
        },
        SatStatus::Unknown => {
            println!("UNKNOWN result. 🤷");
        },
    }

    Ok(())
}
```
### Reading DIMACS CNF Files (with `parser` feature)
The `parser` feature provides functionality to read DIMACS CNF (Conjunctive Normal Form) files, a common format for SAT instances. It leverages the AsDimacs trait to allow parsing directly into a solver or any other structure that implements this trait.

First, ensure you enable the parser feature in your Cargo.toml:
```toml
satgalaxy = { git="https://github.com/sat-galaxy/satgalaxy-rs.git", features = [
    "parser",
    "compression",
] }
```
Then, you can use read_dimacs_from_reader or parse_dimacs_cnf (if you have the content as a string) to load a problem:
```rust
use satgalaxy::{MinisatSolver,SatSolver};
use satgalaxy::parser::{parse_dimacs_cnf, read_dimacs_from_reader, AsDimacs};
use satgalaxy::solver::SatStatus;
use std::fs::File;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Parsing from a string literal
    let dimacs_content = "c This is a comment about the problem
p cnf 3 2
1 -3 0
2 3 0
";

    let mut solver = MinisatSolver::new();
    // `parse_dimacs_cnf` takes a mutable reference to an `AsDimacs` implementor.
    parse_dimacs_cnf(dimacs_content, false, &mut solver).unwrap();

    println!("Solving problem parsed from string:");
    match solver.solve_model().unwrap(){ // Pass an empty slice for assumptions
            SatStatus::Satisfiable(model) => {
                println!("SATISFIABLE! 🎉");
                // Retrieve and print the model (variable assignments)
                println!("Model: {:?} ",model);
            },
            SatStatus::Unsatisfiable => {
                println!("UNSATISFIABLE! 😞");
            },
            SatStatus::Unknown => {
                println!("UNKNOWN result. 🤷");
            },
        }

    // Example 2: Reading from a reader (e.g., a file, or in this case, an in-memory buffer)
    let mut file = File::open("path/to/dimacs_file.cnf").unwrap();
    let mut solver = MinisatSolver::new();

    // `read_dimacs_from_reader` also takes a mutable reference to an `AsDimacs` implementor.
    read_dimacs_from_reader(&mut file,false, &mut solver).unwrap();

    println!("\nSolving problem read from reader:");
    match solver.solve_model().unwrap(){ // Pass an empty slice for assumptions
        SatStatus::Satisfiable(model) => {
            println!("SATISFIABLE! 🎉");
            // Retrieve and print the model (variable assignments)
            println!("Model: {:?} ",model);
        },
        SatStatus::Unsatisfiable => {
            println!("UNSATISFIABLE! 😞");
        },
        SatStatus::Unknown => {
            println!("UNKNOWN result. 🤷");
        },
    }
    Ok(())
}
```
The `AsDimacs` trait is key to this parsing flexibility:
```rust
use satgalaxy::errors::ParserError;
pub trait AsDimacs {
    /// Adds a clause to the underlying structure.
    fn push_clause(&mut self, clause: Vec<i32>)->Result<(),ParserError>;
    /// Adds a comment line. Implementations can choose to store or ignore comments.
    fn add_comment(&mut self, comment: String);
}
```
Currently, `AsDimacs` is implemented for:

- Any type that implements `SatSolver` (like MinisatSolver and GlucoseSolver), allowing you to directly load a DIMACS CNF into a solver.

- `Vec<Vec<i32>>`, which simply collects the clauses into a standard Rust vector.
- `Problem` (defined in `src/parser/mod.rs`), which is a high-level representation of a SAT problem.

## 🧩 Features

satgalaxy-rs leverages Cargo features to allow you to customize which SAT solvers and functionalities are compiled with your project. This helps keep your binary size down and only includes what you need.

You can enable features in your Cargo.toml:

```toml
[dependencies]
satgalaxy = { version="0.2.0", features = [
    "minisat",
    "parser",
    "compression",
    "glucose"
] }
```
Here's a breakdown of the available features:

 - `default`:
       Includes the `minisat`, `parser`, and `glucose` features by default. If you just add `satgalaxy-rs` without specifying features, these will be enabled.
-  `minisat`:
  Enables the Minisat SAT solver backend.
- `glucose`:
        Enables the Glucose SAT solver backend.
- `parser`:
        Enables utilities for parsing standard SAT problem file formats (e.g., DIMACS CNF). This feature depends on the pest and pest_derive crates.
- `compression`:
        Adds support for reading compressed SAT problem files. This feature depends on the `flate2` and `xz2` crates for gzip and xz compression.

## 📜 License

This project is distributed under the MIT License.

## 📧 Contact

If you have any questions, suggestions, or just want to chat about SAT solvers, feel free to open an Issue or reach out. We'd love to hear from you!