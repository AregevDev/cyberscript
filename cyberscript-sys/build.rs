use flate2::read::GzDecoder;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs, io, process};

const CYBER_DL_URL: &'static str =
    "https://github.com/fubark/cyber/archive/refs/tags/latest.tar.gz";

const AR_NAME: &'static str = "cyber-latest.tar.gz";

fn main() -> Result<(), Box<dyn Error>> {
    let zig = which::which("zig").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    // Download source code
    let resp = reqwest::blocking::get(CYBER_DL_URL).unwrap();
    let mut ar_bytes = io::Cursor::new(resp.bytes().unwrap());
    let mut ar_file = fs::File::create(out_dir.join(AR_NAME)).unwrap();
    io::copy(&mut ar_bytes, &mut ar_file).unwrap();

    // Extract
    let ar_file = fs::File::open(out_dir.join(AR_NAME)).unwrap();
    let mut archive = tar::Archive::new(GzDecoder::new(ar_file));
    archive.unpack(&out_dir).unwrap();

    let src_dir = out_dir.join("cyber-latest");

    // Hacky way of converting a Rust target triple to a Zig one
    let first_dash = target.find('-').unwrap();
    let second_dash = target[first_dash + 1..].find('-').unwrap() + first_dash + 1;
    let zig_target = format!("{}{}", &target[..first_dash], &target[second_dash..]);

    // Build
    let status = process::Command::new(zig)
        .current_dir(&src_dir)
        .arg("build")
        .arg("lib")
        .arg(format!("-Dtarget={}", zig_target))
        .arg("-Doptimize=ReleaseFast")
        .status()
        .unwrap();

    assert!(status.success());

    // Generate bindings
    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", src_dir.join("src/include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        src_dir.join("zig-out/lib").display()
    );
    println!("cargo:rustc-link-lib=cyber");

    Ok(())
}
