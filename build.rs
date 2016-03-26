use std::env::{var, remove_var};
use std::path::{PathBuf};
use std::process::Command;

const BLIS_SRC: &'static str = "blis";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", BLIS_SRC);
    let use_ccache = cfg!(feature = "ccache");
    let plat = var("RBLIS_CONFIG").unwrap_or(String::from("auto"));
    let kind = "static";

    let crate_home = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap());
    let blis_source = crate_home.join(BLIS_SRC);
    let output = PathBuf::from(&var("OUT_DIR").unwrap());

    if cfg!(feature = "system") {
        let kind = "dylib";
        println!("cargo:rustc-link-lib={}=blis", kind);
        return;
    }
    
    // always use pthreads so that it is thread safe to call

    remove_var("TARGET");
    
    // configure with output to $prefix/lib/libblis.a
    run(Command::new(&blis_source.join("configure"))
                .arg("-p") // set prefix to install to
                .arg(&output)
                .args(&["-t", "pthreads"])
                .arg(&plat)
                .current_dir(&output));

    let mut ccache_arg = &["CC=ccache gcc"][..];
    if !use_ccache {
        ccache_arg = &[];
    }
    run(Command::new("make")
                .arg(&format!("-j{}", var("NUM_JOBS").unwrap()))
                .args(ccache_arg)
                .current_dir(&output));

    run(Command::new("make")
                .arg("install")
                .current_dir(&output));

    println!("cargo:rustc-link-search={}", output.join("lib").display());
    println!("cargo:rustc-link-lib={}=blis", kind);
}

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{:?}` failed: {}", command, status);
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        },
    }
}
