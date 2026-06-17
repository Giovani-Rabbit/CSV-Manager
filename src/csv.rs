use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::json::{json_serialize, pair_headers_with_records};

#[derive(Debug, PartialEq)]
pub struct Csv {
    pub headers: Vec<String>,
    pub lines: Vec<Vec<String>>,
}

impl Csv {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let first_line = Self::get_first_line(file)?;

        let delimiter = detect_delimiter(&first_line)?;

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter.as_byte())
            .has_headers(true)
            .from_path(path)?;

        let headers = reader
            .headers()?
            .iter()
            .map(|col| col.trim().replace("\"", ""))
            .filter(|col| !col.is_empty())
            .collect();

        let lines: Vec<Vec<String>> = reader
            .records()
            .map(|r| {
                r.expect(&format!(
                    "failed to read record!! delimiter used = '{}'",
                    delimiter.as_char(),
                ))
                .iter()
                .map(|field| field.trim().to_string())
                .filter(|field| !field.is_empty())
                .collect()
            })
            .collect();

        Ok(Self { headers, lines })
    }

    fn get_first_line(file: File) -> Result<String, Box<dyn Error>> {
        let mut buf = BufReader::new(file);
        let mut first_line = String::new();
        buf.read_line(&mut first_line)?;

        Ok(first_line)
    }

    fn column_index(&self, column: &str) -> Option<usize> {
        self.headers
            .iter()
            .position(|c| c.eq_ignore_ascii_case(column))
    }

    pub fn filter(&self, condition: &str) -> Result<Vec<Vec<String>>, String> {
        let (column, operator, value) = extract_condition(condition)?;
        let column_position = self
            .column_index(&column)
            .ok_or(format!("Column '{column}' not found"))?;

        let result: Vec<Vec<String>> = self
            .lines
            .iter()
            .filter(|line| {
                if let Some(field_value) = line.get(column_position) {
                    compare_values(field_value, &operator, &value)
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        Ok(result)
    }

    pub fn to_json(&self) -> String {
        let mapped_records = pair_headers_with_records(&self.headers, &self.lines);
        json_serialize(&mapped_records)
    }
}

fn compare_values(field_value: &str, operator: &str, value: &str) -> bool {
    match operator {
        "==" => field_value.eq_ignore_ascii_case(value),
        "!=" => !field_value.eq_ignore_ascii_case(value),
        ">" => {
            if let (Ok(a), Ok(b)) = (field_value.parse::<f64>(), value.parse::<f64>()) {
                a > b
            } else {
                field_value > value
            }
        }
        "<" => {
            if let (Ok(a), Ok(b)) = (field_value.parse::<f64>(), value.parse::<f64>()) {
                a < b
            } else {
                field_value < value
            }
        }
        ">=" => {
            if let (Ok(a), Ok(b)) = (field_value.parse::<f64>(), value.parse::<f64>()) {
                a >= b
            } else {
                field_value >= value
            }
        }
        "<=" => {
            if let (Ok(a), Ok(b)) = (field_value.parse::<f64>(), value.parse::<f64>()) {
                a <= b
            } else {
                field_value <= value
            }
        }
        _ => false,
    }
}

fn extract_condition(c: &str) -> Result<(String, String, String), String> {
    let operators = [">=", "<=", "!=", "==", ">", "<"];

    for op in operators {
        if let Some(idx) = c.find(op) {
            let column = c[..idx].trim().to_string();
            if column.is_empty() {
                return Err(format!("column name is missing in '{c}'"));
            }

            let value = c[idx + op.len()..].trim().to_string();
            if value.is_empty() {
                return Err(format!("value of operation is missing in '{c}'"));
            }

            return Ok((column, op.to_string(), value));
        }
    }

    Err(format!(
        "No valid operator could be found. Try using {} Ex: 'age>18'",
        operators.join(" ")
    ))
}

#[derive(Debug)]
enum Delimiter {
    Semicolon,
    Comma,
    Tab,
    Pipe,
}

impl Delimiter {
    fn as_char(&self) -> char {
        match self {
            Delimiter::Semicolon => ';',
            Delimiter::Comma => ',',
            Delimiter::Tab => '\t',
            Delimiter::Pipe => '|',
        }
    }

    fn as_byte(&self) -> u8 {
        match self {
            Delimiter::Semicolon => b';',
            Delimiter::Comma => b',',
            Delimiter::Tab => b'\t',
            Delimiter::Pipe => b'|',
        }
    }
}

fn detect_delimiter(header: &str) -> Result<Delimiter, String> {
    let candidates = [
        Delimiter::Semicolon,
        Delimiter::Comma,
        Delimiter::Tab,
        Delimiter::Pipe,
    ];

    candidates
        .into_iter()
        .max_by_key(|d| header.chars().filter(|&c| c == d.as_char()).count())
        .ok_or_else(|| format!("could not detect delimiter in '{header}'"))
}

#[cfg(test)]
mod tests {
    use crate::csv::{compare_values, extract_condition};

    #[test]
    fn should_extract_column_operator_and_value() {
        let input = "amount>20";

        let (column, operator, value) = extract_condition(input).unwrap();

        assert_eq!(column, "amount");
        assert_eq!(operator, ">");
        assert_eq!(value, "20");
    }

    #[test]
    fn should_return_error_when_operator_is_missing() {
        let input = "amount20";

        let error = extract_condition(input).unwrap_err();

        assert!(error.contains("No valid operator could be found."));
    }

    #[test]
    fn should_return_error_when_column_is_missing() {
        let input = ">20";

        let error = extract_condition(input).unwrap_err();

        assert!(error.contains("column name is missing"));
    }

    #[test]
    fn should_return_error_when_value_is_missing() {
        let input = "amount>";

        let error = extract_condition(input).unwrap_err();

        assert!(error.contains("value of operation is missing"));
    }

    #[test]
    fn should_compare_numeric_values_correctly() {
        assert!(compare_values("30", ">", "20"));
        assert!(compare_values("30", "==", "30"));
        assert!(compare_values("20", "<", "40"));
        assert!(compare_values("40", "<=", "40"));
        assert!(compare_values("40", ">=", "40"));
        assert!(compare_values("20", "!=", "40"));

        assert!(!compare_values("30", "==", "20"));
        assert!(!compare_values("20", "!=", "20"));
    }

    #[test]
    fn should_compare_strings_case_insensitively() {
        assert!(compare_values("rust", "==", "rust"));
        assert!(compare_values("RUST", "==", "rust"));
        assert!(compare_values("rust", "!=", "golang"));

        assert!(!compare_values("Rust", "!=", "rust"));
    }
}
