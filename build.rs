use std::{env, path::Path};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(armv7a)");

    let target = env::var("TARGET").unwrap();
    if target.starts_with("armv7a-none") {
        println!("cargo:rustc-cfg=armv7a");
    }

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }

    let dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cortex-a-rt.x");
    println!("cargo:rustc-link-search={}", dir.display());
}
