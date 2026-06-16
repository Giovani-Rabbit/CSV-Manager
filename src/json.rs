pub fn pair_headers_with_records(
    headers: &[String],
    records: &[Vec<String>],
) -> Vec<Vec<(String, String)>> {
    let mut json_map: Vec<Vec<(String, String)>> = Vec::with_capacity(records.len());

    for record in records {
        let mut line: Vec<(String, String)> = Vec::with_capacity(headers.len());

        for (idx, value) in record.iter().enumerate() {
            line.push((headers[idx].clone(), value.clone()));
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
            let value = add_quotes(&record.1);
            object.push(concat_key_value(&key, &value));
        }

        json.push(add_brackets(&object.join(", ")));
    }

    format!("[{}]", json.join(", "))
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
