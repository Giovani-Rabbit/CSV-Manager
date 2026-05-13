use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
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
            .map(|col| col.trim().to_string())
            .collect();

        println!("header: {:?}", headers);

        let lines: Vec<Vec<String>> = reader
            .records()
            .map(|r| {
                r.expect(&format!(
                    "failed to read record!! delimiter used = '{}'",
                    delimiter.as_char(),
                ))
                .iter()
                .map(|field| field.trim().to_string())
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

    pub fn filter(&self, condition: &str) -> Result<(), String> {
        let (column, operator, value) = extract_condition(condition)?;

        // preciso pegar o valor da coluna, verificar se o valor eh um numero
        // e entao fazer a comparacao com os operadores.
        println!("header: {:?}", self.headers);
        println!("lines: {:?}", self.lines);

        let column_value = self.column_index(&column).unwrap();

        println!("nome da coluna: {column} | valor da colunas: {column_value}");

        todo!()
    }
}

fn compare_values(field: &str, operator: &str, value: f64) {}

fn extract_condition(c: &str) -> Result<(String, String, f64), String> {
    let operators = [">=", "<=", "!=", "==", ">", "<"];

    for op in operators {
        if let Some(idx) = c.find(op) {
            let column = c[..idx].trim().to_string();

            if column.is_empty() {
                return Err(format!("column name is missing in '{c}'"));
            }

            let value = c[idx + op.len()..].trim();
            let int_value = value
                .parse::<f64>()
                .map_err(|_| format!("'{value}' is not a number"))?;

            return Ok((column, op.to_string(), int_value));
        }
    }

    Err(format!("no operator found in '{c}', expected ex: age>18"))
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
