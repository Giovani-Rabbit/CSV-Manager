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
fn test_csv_show() {
    let output = Command::new("cargo")
        .args(["run", CSV_FILE, "show"])
        .output()
        .expect("falhou ao executar");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout não é UTF-8 válido");

    assert!(stdout.contains("Username  | Identifier | First name | Last name"));
    assert!(stdout.contains("----------+------------+------------+----------"));
    assert!(stdout.contains("johnson81 | 4081       | Craig      | Johnson"));
}

#[test]
fn test_csv_json() {
    let output = Command::new("cargo")
        .args(["run", CSV_FILE, "export"])
        .output()
        .expect("falhou ao executar");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout não é UTF-8 válido");

    assert!(stdout.contains("\"Craig\""));
    assert!(stdout.contains("\"Username\": \"grey07\""));
    assert!(stdout.contains("\"Last name\": \"Smith\""));
}
