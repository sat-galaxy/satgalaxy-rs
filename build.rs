use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};


fn binding_cadical(version: &str) {
   
}
fn binding_minisat() {
    let minisat_dir = "satgalaxy-core/minisat";
    let mut cfg=cmake::Config::new(minisat_dir);
    let dst =    cfg.build_target("minisat-lib-static")
       .build() ;
    // let dst = cmake::build(&minisat_dir);
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=minisat");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", minisat_dir);

    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/minisat/external/minisat_port.h",
            minisat_dir
        )])
        .allowlist_function("minisat_.*")
        // .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("minisat_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn binding_glucose() {
   
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
        binding_glucose();
    }
}
