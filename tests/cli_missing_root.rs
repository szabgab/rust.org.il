use std::process::Command;

#[test]
fn missing_root_argument_returns_error_without_panic() {
    let mut missing_root = std::env::temp_dir();
    missing_root.push(format!(
        "data-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("current time should be after unix epoch")
            .as_nanos()
    ));

    // Ensure the path does not exist.
    assert!(
        !missing_root.exists(),
        "trouble in test setup, temp directory already exists: {missing_root:?}"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rust-israel"))
        .arg(&missing_root)
        .output()
        .expect("failed to run rust-israel binary");

    assert!(
        !output.status.success(),
        "expected non-zero exit code for missing root"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("Data root") && stderr.contains("does not exist"),
        "expected friendly missing root error, got: {stderr}"
    );
    assert!(
        !stderr.contains("panicked at"),
        "expected no panic output, got: {stderr}"
    );
}
