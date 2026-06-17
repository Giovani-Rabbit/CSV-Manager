pub fn pair_headers_with_records(
    headers: &[String],
    records: &[Vec<String>],
) -> Vec<Vec<(String, String)>> {
    let mut json_map: Vec<Vec<(String, String)>> = Vec::with_capacity(records.len());

    for record in records {
        let mut line: Vec<(String, String)> = Vec::with_capacity(headers.len());

        for (idx, header) in headers.iter().enumerate() {
            line.push((header.clone(), record[idx].clone()));
        }

        json_map.push(line);
    }

    json_map
}

pub fn json_serialize(csv_mapped: &[Vec<(String, String)>]) -> String {
    let mut json: Vec<String> = Vec::with_capacity(csv_mapped.len());

    for records in csv_mapped {
        let mut object: Vec<String> = Vec::with_capacity(records.len());

        for record in records {
            let key = add_quotes(&record.0);
            let value = format_value(&record.1);
            object.push(concat_key_value(&key, &value));
        }

        json.push(add_brackets(&object.join(", ")));
    }

    format!("[{}]", json.join(", "))
}

fn format_value(v: &str) -> String {
    if is_numeric(v) || is_boolean(v) {
        v.to_string()
    } else {
        add_quotes(v)
    }
}

fn concat_key_value(key: &str, value: &str) -> String {
    format!("{key}: {value}")
}

fn add_quotes(v: &str) -> String {
    format!("\"{}\"", v)
}

fn add_brackets(v: &str) -> String {
    format!("{{{}}}", v)
}

fn is_boolean(v: &str) -> bool {
    matches!(v.to_lowercase().as_str(), "true" | "false")
}

fn is_numeric(v: &str) -> bool {
    !v.is_empty()
        && v.chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',' || c == '-')
        && (v.parse::<f64>().is_ok() || v.contains(","))
}
