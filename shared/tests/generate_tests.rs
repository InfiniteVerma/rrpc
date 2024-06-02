// tests/generate_tests.rs

use std::process::Command;

#[test]
fn test_generate_binary() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "generate"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("The result is: 5"));
}

