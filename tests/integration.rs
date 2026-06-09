use std::process::Command;

const CSV_FILE: &str = "./tests/data.csv";

#[test]
fn test_csv_filter() {
    let output = Command::new("cargo")
        .args(["run", CSV_FILE, "filter", "Identifier>2000"])
        .output()
        .expect("falhou ao executar");

    assert!(output.status.success());
}

#[test]
fn test_csv_filter_invalid_operator() {
    let output = Command::new("cargo")
        .args(["run", CSV_FILE, "filter", "Identifier1000"])
        .output()
        .unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("stderr: {stderr}");

    assert!(!output.status.success());
    assert!(stderr.contains("No valid operator could be found"))
}
