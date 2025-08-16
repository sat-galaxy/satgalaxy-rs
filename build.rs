use std::env;
use std::path::{Path, PathBuf};

struct SatBuild<'a> {
    base_dir: String,
    name: &'a str,
    build: cc::Build,
}
impl<'a> SatBuild<'a> {
    fn new(base_dir: &'a str, name: &'a str) -> Self {
        let real_base_dir=format!("satgalaxy-core/{}",base_dir);
        let mut build = cc::Build::new();
        build.warnings(false);
        build.include(real_base_dir.as_str());
        build.out_dir(PathBuf::from(env::var("OUT_DIR").unwrap()).join(base_dir));

        Self {
            base_dir:real_base_dir,
            name,
            build,
        }
    }
    fn include(&mut self, dir: &'a str) -> &mut Self {
        self.build
            .include(format!("{}/{}", self.base_dir, dir).as_str());
        self
    }
    fn files<P>(&mut self, files: P) -> &mut Self
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        self.build.files(
            files
                .into_iter()
                .map(|p| format!("{}/{}", self.base_dir, p.as_ref().display())),
        );
        self
    }
    fn define<'b, V: Into<Option<&'b str>>>(&mut self, var: &str, val: V) -> &mut Self {
        self.build.define(var, val);
        self
    }
    fn flags(&mut self, flags: &[&str]) -> &mut Self {
        flags.iter().for_each(|f| {
            self.build.flag_if_supported(f);
        });
        self
    }
    fn flag(&mut self, flag: &str) -> &mut Self {
        self.build.flag_if_supported(flag);
        self
    }
    fn cpp(&mut self, cpp: bool) -> &mut Self {
        self.build.cpp(cpp);
        self
    }
    fn build(&mut self, header: &str) {
        self.build
            .compile(format!("satgalaxy_{}", self.name).as_str());
        // let bindings = bindgen::Builder::default()
        //     .headers([format!("{}/{}", self.base_dir, header)])
        //     .allowlist_function(format!("{}_.*", self.name))
        //     .generate_comments(true)
        //     .generate()
        //     .expect("Unable to generate bindings");

        // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        // bindings
        //     .write_to_file(out_path.join(format!("{}_bindings.rs", self.name)))
        //     .expect("Couldn't write bindings!");
    }
}

fn binding_glucose(version: &str) {
    let path = format!("glucose-{}", version);
    let sources = [
        "utils/Options.cc",
        "core/Solver.cc",
        "external/satgalaxy_glucose.cc",
        "core/lcm.cc",
        "simp/SimpSolver.cc",
    ];
    let mut build = SatBuild::new(&path, "glucose");
    build
        .files(sources)
        .cpp(true)
        .build("external/satgalaxy_glucose.h");
}
fn binding_cadical(version: &str) {
    let base_dir = format!("cadical-rel-{}", version);

    let sources = [
        "external/satgalaxy_cadical.cc",
        "src/solver.cpp",
        "src/internal.cpp",
        "src/external.cpp",
        "src/message.cpp",
        "src/report.cpp",
        "src/lookahead.cpp",
        "src/decompose.cpp",
        "src/clause.cpp",
        "src/collect.cpp",
        "src/propagate.cpp",
        "src/decide.cpp",
        "src/var.cpp",
        "src/proof.cpp",
        "src/arena.cpp",
        "src/analyze.cpp",
        "src/flags.cpp",
        "src/extend.cpp",
        "src/external_propagate.cpp",
        "src/minimize.cpp",
        "src/shrink.cpp",
        "src/reap.cpp",
        "src/solution.cpp",
        "src/ema.cpp",
        "src/probe.cpp",
        "src/deduplicate.cpp",
        "src/ternary.cpp",
        "src/watch.cpp",
        "src/config.cpp",
        "src/contract.cpp",
        "src/options.cpp",
        "src/version.cpp",
        "src/limit.cpp",
        "src/assume.cpp",
        "src/constrain.cpp",
        "src/lratbuilder.cpp",
        "src/stats.cpp",
        "src/queue.cpp",
        "src/score.cpp",
        "src/backtrack.cpp",
        "src/restart.cpp",
        "src/elim.cpp",
        "src/subsume.cpp",
        "src/instantiate.cpp",
        "src/flip.cpp",
        "src/rephase.cpp",
        "src/backward.cpp",
        "src/condition.cpp",
        "src/averages.cpp",
        "src/gates.cpp",
        "src/block.cpp",
        "src/phases.cpp",
        "src/reduce.cpp",
        "src/compact.cpp",
        "src/walk.cpp",
        "src/lucky.cpp",
        "src/util.cpp",
        "src/restore.cpp",
        "src/checker.cpp",
        "src/occs.cpp",
        "src/cover.cpp",
        "src/bins.cpp",
        "src/vivify.cpp",
        "src/transred.cpp",
        "src/lratchecker.cpp",
    ];

    SatBuild::new(base_dir.as_str(), "cadical")
        .files(sources)
        .include("src")
        .cpp(true)
        .define("QUIET", None)
        .define("GALAXY_CORE", None)
        .define("NTRACING", None)
        .define("VERSION", format!("\"{}\"", version).as_str())
        .flag("-Wno-error=date-time")
        .build("external/satgalaxy_cadical.h");
}
fn binding_minisat() {
    let path = "minisat";
    let sources = [
        "minisat/utils/Options.cc",
        "minisat/core/Solver.cc",
        "minisat/external/satgalaxy_minisat.cc",
        "minisat/simp/SimpSolver.cc",
    ];
    SatBuild::new(path, "minisat")
        .files(sources)
        .cpp(true)
        .define("__STDC_FORMAT_MACROS", None)
        .define("__STDC_LIMIT_MACROS", None)
        .build("minisat/external/satgalaxy_minisat.h");
}
fn binding_picosat(version: &str) {
    let path = format!("picosat-{}", version);
    let sources = ["picosat.c", "external/satgalaxy_picosat.c"];
    let mut build = SatBuild::new(path.as_str(), "picosat");
    build.files(sources).cpp(false).define("NGETRUSAGE", None);
    if cfg!(feature = "trace") {
        build.define("TRACE", None);
    }
    build.build("external/satgalaxy_picosat.h");
}
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if cfg!(feature = "cadical") {
        binding_cadical("2.1.3");
    }
    if cfg!(feature = "glucose") {
        binding_glucose("4.2.1");
    }
    if cfg!(feature = "minisat") {
        binding_minisat();
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
