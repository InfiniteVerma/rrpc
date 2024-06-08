use std::process::Command;
use std::path::Path;

fn main() {
    let input_file = "input.txt";
    let out_dir = "src/generated/";

    if !Path::new("../target/debug/rrpc").is_file() {
        panic!("rrpc binary is not present!");
    }

    let status = Command::new("../target/debug/rrpc")
        .arg(input_file)
        .arg(out_dir)
        .status()
        .expect("Failed to run rrpc");

    if !status.success() {
        panic!("Failed to generate files");
    }

    println!("cargo:rerun-if-changed={}", input_file);
    //println!("cargo:rerun-if-changed=build.rs");
}
