// tests/generate_tests.rs

use std::process::Command;
use tempfile::tempdir;
use std::path::Path;
use std::fs;

#[test]
fn test_generate_with_valid_file() {
    let test_data_dir = Path::new("tests/data/inputs");

    for entry in fs::read_dir(test_data_dir).expect("Failed to read data dir") {
        let entry = entry.expect("Failed to read directory entry");
        let test_file_path = entry.path();

        // run test
        if test_file_path.is_file() {

            let temp_dir = tempdir().expect("Failed to create temp dir");
            let temp_file_path = temp_dir.path().join(test_file_path.file_name().unwrap());

            fs::copy(&test_file_path, &temp_file_path).expect("Failed to copy to temp test file");

            let output = Command::new("cargo")
                .args(&["run", "--bin", "generate", "--", temp_file_path.to_str().unwrap(), temp_dir.path().to_str().unwrap()])
                .output()
                .expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
            //assert!(output.status.success());

            let out_file = temp_dir.path().join("gen.rs");
            let out_file_contents = fs::read_to_string(&out_file).expect("Failed to read from gen.rs");
            println!("{}", out_file_contents);

            let expected_output_file = Path::new("tests/data/expected")
                .join(test_file_path.file_name().unwrap())
                .with_extension("gen.rs");

            println!("Expected file: {}", expected_output_file.to_str().unwrap());

            let expected_output = fs::read_to_string(&expected_output_file).expect("Failed to read from expected file");

            assert_eq!(out_file_contents, expected_output);
        }
    }
}
