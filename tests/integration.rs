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
