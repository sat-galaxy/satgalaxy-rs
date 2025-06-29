use std::path::PathBuf;
use std::env;

fn binding_cadical(_version: &str) {}
fn binding_minisat() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let minisat_dir = "satgalaxy-core/minisat";
    let mut cfg = cmake::Config::new(minisat_dir);
    let dst = cfg
        .out_dir(out_path.join("minisat"))
        .build_target("minisat-lib-static")
        .build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=minisat");
    println!("cargo:rerun-if-changed={}", minisat_dir);

    let bindings = bindgen::Builder::default()
        .headers([format!("{}/minisat/external/minisat_port.h", minisat_dir)])
        .allowlist_function("minisat_.*")
        // .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("minisat_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn binding_glucose(version: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let solver_dir = format!("satgalaxy-core/glucose-{}", version);
    let mut cfg = cmake::Config::new(&solver_dir);
    let dst = cfg
        .out_dir(out_path.join(format!("glucose-{}", version)))
        .build_target("glucose")
        .build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=glucose");
    println!("cargo:rerun-if-changed={}", &solver_dir);

    let bindings = bindgen::Builder::default()
        .headers([format!("{}/external/glucose_port.h", solver_dir)])
        .allowlist_function("glucose_.*")
        // .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glucose_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if cfg!(feature = "cadical") {
        binding_cadical("2.1.3");
    }
    if cfg!(feature = "minisat") {
        binding_minisat();
    }
    if cfg!(feature = "glucose") {
        binding_glucose("4.2.1");
    }
    let target = env::var("TARGET").unwrap();
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }
}
