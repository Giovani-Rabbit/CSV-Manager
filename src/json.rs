pub fn csv_to_json(headers: &[String], records: &[Vec<String>]) -> Vec<Vec<(String, String)>> {
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
