use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    
    // 获取内核版本信息
    let kernel_dir = "/home/julyfun/Documents/GitHub/linux";
    
    // 设置环境变量供编译时使用
    println!("cargo:rustc-link-search=native={}/samples/rust", kernel_dir);
    
    // 生成模块版本信息
    let version_output = Command::new("make")
        .args(["-s", "-C", kernel_dir, "kernelversion"])
        .output()
        .expect("Failed to get kernel version");
    let version = String::from_utf8_lossy(&version_output.stdout).trim().to_string();
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let version_file = PathBuf::from(&out_dir).join("version.rs");
    
    fs::write(&version_file, format!("
        pub const KERNEL_VERSION: &str = \"{}\";
    ", version)).expect("Failed to write version file");
    
    println!("cargo:rustc-env=KERNEL_VERSION={}", version);
}
