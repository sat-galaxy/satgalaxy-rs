use std::io::Read;
use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

use patch::Patch;

fn binding_cadical() {
    // 设置 CaDiCaL 的源目录
    let cadical_dir = PathBuf::from("third_parts/cadical");

    // 运行 configure
    let status = Command::new("sh")
        .current_dir(&cadical_dir)
        .arg("configure")
        .status()
        .expect("Failed to execute configure");

    if !status.success() {
        panic!("Failed to configure CaDiCaL");
    }

    // 运行 make
    let status = Command::new("make")
        .current_dir(&cadical_dir)
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build CaDiCaL");
    }
    // 设置链接搜索路径
    println!(
        "cargo:rustc-link-search=native={}/build",
        cadical_dir.display()
    );
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=stdc++");

    // 告诉 cargo 如果这些文件改变了就重新运行脚本
    println!(
        "cargo:rerun-if-changed={}/src/cadical.hpp",
        cadical_dir.display()
    );

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/src/cadical.hpp", cadical_dir.display()))
        .allowlist_type("CaDiCaL::Solver")
        .allowlist_function("CaDiCaL::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    // 写入绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cadical_bindings.rs"))
        .expect("Couldn't write bindings!");
}
fn binding_minisat() {
    let minisat_dir = PathBuf::from("third_parts/minisat");
    apply_patch(&minisat_dir,"minisat.patch");
    let dst = cmake::build(&minisat_dir);
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=minisat");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", minisat_dir.display());

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .headers([format!(
            "{}/include/minisat/simp/StdSimpSolver.hpp",
            dst.display()
        )])
        // .headers(["StdSimpSolver.hpp"])
        // .allowlist_type("Minisat::SimpSolver")
        .allowlist_function("Minisat::.*")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    // 写入绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("minisat_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn binding_glucose() {
    
    let glucose_dir = PathBuf::from("third_parts/glucose");
    apply_patch(&glucose_dir,"glucose.patch");
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

    // 写入绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glucose_bindings.rs"))
        .expect("Couldn't write bindings!");
}


fn apply_patch(dir: &PathBuf, patch_name: &str) {
    let patch_path = PathBuf::from("patches").join(patch_name);
    if patch_path.exists() {
        let status = Command::new("git")
            // .current_dir(dir)
            .arg("apply")
            .arg("--ignore-whitespace")
            .arg("--directory")
            .arg(dir)
            .arg(patch_path)
            .status()
            .expect("Failed to execute git apply");
        if !status.success() {
            println!("cargo::warning=Failed to apply patch: {}", patch_name);
        }
    }else {
        panic!("Failed to apply patch: {}", patch_name);
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
