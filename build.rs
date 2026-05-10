use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let hook_dll_path = env::var("CARGO_CDYLIB_FILE_HOOK_hook").unwrap();
    println!("cargo:rerun-if-changed={}", hook_dll_path);

    let hook_dll = PathBuf::from(&hook_dll_path);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let compressed = out_dir.join("hook_compressed.dll");
    std::fs::copy(&hook_dll, &compressed).unwrap();

    let status = Command::new("upx")
        .args(&match std::env::var("PROFILE").unwrap().as_str() {
            "release" => ["--ultra-brute", "--lzma", "--best"].to_vec(),
            _ => ["--no-lzma", "-1"].to_vec(),
        })
        .arg("-k")
        .arg(compressed.to_str().unwrap())
        .status().unwrap();

    if !status.success() {
        panic!("UPX compression failed");
    }

    println!("cargo:rustc-env=HOOK_DLL_PATH={}", compressed.display());
}
