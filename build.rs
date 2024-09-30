use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use flate2::read::GzDecoder;
use tar::Archive;
const MINISAT_URL: &str = "https://github.com/niklasso/minisat/archive/refs/heads/master.tar.gz";
const CADICAL_URL: &str =
    "https://github.com/arminbiere/cadical/archive/refs/tags/rel-2.0.0.tar.gz";
const GLUCOSE_URL: &str = "https://github.com/audemard/glucose/archive/refs/tags/4.2.1.tar.gz";

fn get_extract_dir(out_dir: &PathBuf) -> PathBuf {
    fs::read_dir(&out_dir)
        .expect("Failed to read extracted directory")
        .filter_map(Result::ok)
        .find(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|entry| entry.path())
        .expect("Failed to find extracted solver directory")
}

fn download_and_extract(url: &str, name: &str) -> PathBuf {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_dir = out_path.join("third_parts").join(name);
    let out_dir_ref = &out_dir;
    let complete_file = out_dir_ref.join(".complete");
    if complete_file.exists() {
        return get_extract_dir(out_dir_ref);
    }
    let response = reqwest::blocking::get(url).expect("Failed to download solver");
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);
    fs::create_dir_all(out_dir_ref).expect("Failed to create directory");
    archive
        .unpack(out_dir_ref)
        .expect("Failed to extract solver");
    // Find the extracted directory (it might have a version suffix)
    let d = get_extract_dir(out_dir_ref);
    apply_patch(&d, name);
    fs::write(&complete_file, b"").expect("Failed to create .complete file");
    d
}

fn binding_cadical() {
    let cadical_dir = download_and_extract(CADICAL_URL, "cadical");

    let status = Command::new("sh")
        .current_dir(&cadical_dir)
        .arg("configure")
        .status()
        .expect("Failed to execute configure");

    if !status.success() {
        panic!("Failed to configure CaDiCaL");
    }

    let status = Command::new("make")
        .current_dir(&cadical_dir)
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build CaDiCaL");
    }
    println!(
        "cargo:rustc-link-search=native={}/build",
        cadical_dir.display()
    );
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=stdc++");

    println!(
        "cargo:rerun-if-changed={}/src/cadical.hpp",
        cadical_dir.display()
    );

    let bindings = bindgen::Builder::default()
        .header(format!("{}/src/cadical.hpp", cadical_dir.display()))
        .allowlist_type("CaDiCaL::Solver")
        .allowlist_function("CaDiCaL::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cadical_bindings.rs"))
        .expect("Couldn't write bindings!");
}
fn binding_minisat() {
    let minisat_dir = download_and_extract(MINISAT_URL, "minisat");
    let dst = cmake::build(&minisat_dir);
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=minisat");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", minisat_dir.display());

    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/include/minisat/simp/StdSimpSolver.hpp",
            dst.display()
        )])
        .allowlist_function("Minisat::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("minisat_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn binding_glucose() {
    let glucose_dir = download_and_extract(GLUCOSE_URL, "glucose");
    let dst = cmake::build(&glucose_dir);
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=glucose");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", glucose_dir.display());

    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/include/glucose/simp/StdSimpSolver.hpp",
            dst.display()
        )])
        .allowlist_function("Glucose::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glucose_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn apply_patch(dir: &PathBuf, patch_name: &str) {
    let patch_path = PathBuf::from("patches").join(format!("{}.patch", patch_name));
    if patch_path.exists() {
        let abs_path = fs::canonicalize(&patch_path).unwrap();
        let status = Command::new("patch")
            .arg("-f")
            .arg("-r")
            .arg(".ignore")
            .arg("--ignore-whitespace")
            .arg("--directory")
            .arg(dir)
            .arg("-p1")
            .arg("-i")
            .arg(abs_path)
            .status()
            .expect("Failed to execute git apply");
        if !status.success() {
            println!("cargo::warning=Failed to apply patch: {}", patch_name);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if cfg!(feature = "cadical") {
        binding_cadical();
    }
    if cfg!(feature = "minisat") {
        binding_minisat();
    }
    if cfg!(feature = "glucose") {
        binding_glucose();
    }
}
