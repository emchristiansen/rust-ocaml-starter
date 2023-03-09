use std::{env, path::PathBuf};

pub fn run() {
    let root = {
        let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        PathBuf::from(current_dir)
    };

    assert!(std::process::Command::new("dune")
        .current_dir(&root)
        .arg("build")
        .arg("--build-dir")
        .arg("_build")
        // .arg("--release")
        .status()
        .unwrap()
        .success());

    let mut build = cc::Build::new();
    let path = root
        .join("_build")
        .join("default")
        .join("lib")
        .join("foo.so");
    build.object(path);

    build.compile("ocaml");
}

pub fn main() {
    run();
}
