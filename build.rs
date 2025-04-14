use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("Building Orion with profile: {}", profile);

    // Create a directories module to make it easier to find config files
    let dest_path = Path::new(&out_dir).join("directories.rs");
    fs::write(
        &dest_path,
        r#"
pub fn get_config_dir() -> std::path::PathBuf {
    let proj_dirs = directories::ProjectDirs::from("", "", "orion")
        .expect("Could not determine config directory");
    let config_dir = proj_dirs.config_dir().to_path_buf();
    // Ensure the directory exists
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir
}

pub fn get_socket_path() -> std::path::PathBuf {
    get_config_dir().join("orion.sock")
}

pub fn get_config_path() -> std::path::PathBuf {
    get_config_dir().join("config.toml")
}
"#,
    ).unwrap();

    println!("Building optimized release binaries...");

    // Compile all workspace members with optimizations
    let status = Command::new("cargo")
        .args(&["build", if profile == "release" { "--release" } else { "" }, "--all"])
        .status()
        .expect("Failed to build workspace members");

    if !status.success() {
        panic!("Build failed");
    }

    println!("Build completed successfully!");
    println!("Binaries available in: target/{}/", profile);
}
