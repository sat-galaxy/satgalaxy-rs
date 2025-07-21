use std::env;
use std::path::PathBuf;

fn binding_satsolver(path: &str, name: &str,defines:Vec<(&str, &str)>) {
    let solver_dir = format!("satgalaxy-core/{}", path);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut cfg = cmake::Config::new(&solver_dir);
    defines.iter().for_each(|(k,v)| {cfg.define(k,v);});
    let dst = cfg.out_dir(out_path.join(name)).build_target(name).build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=satgalaxy_{}", name);
    println!("cargo:rerun-if-changed={}", &solver_dir);

    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/build/include/satgalaxy/satgalaxy_{}.h",
            dst.display(),
            name
        )])
        .allowlist_function(format!("{}_.*", name))
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}_bindings.rs", name)))
        .expect("Couldn't write bindings!");
}

fn binding_glucose(version: &str) {
    let path = format!("glucose-{}", version);
    binding_satsolver(&path, "glucose",Default::default());
}
fn binding_cadical(version: &str) {
     let path = format!("cadical-rel-{}", version);
    binding_satsolver(&path, "cadical",Default::default());
}
fn binding_minisat() {
    binding_satsolver("minisat", "minisat",Default::default());
}
fn binding_picosat(version: &str) {
    let path = format!("picosat-{}", version);
    let mut defines=vec![];
    if cfg!(feature = "trace") {
        defines.push(("USE_TRACE", "ON"));
    } else {
        defines.push(("USE_TRACE", "OFF"));
    }
    binding_satsolver(&path, "picosat", defines);
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
    if cfg!(feature = "picosat") {
        binding_picosat("960");
    }
    let target = env::var("TARGET").unwrap();
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }
}
