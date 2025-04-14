use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Get target directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    println!("Building optimized release binaries...");

    // Compile all workspace members with optimizations
    let status = Command::new("cargo")
        .args(&["build", "--release", "--all"])
        .status()
        .expect("Failed to build workspace members");

    if !status.success() {
        panic!("Build failed");
    }

    println!("Build completed successfully!");

    // Output information about the location of compiled binaries
    println!("Binaries available in: target/release/");
}
