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
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',')
        && (v.parse::<f64>().is_ok() || v.contains(","))
}

#[cfg(test)]
mod json_test {
    use super::*;

    #[test]
    fn should_pair_header_with_records() {
        let headers = vec!["name".to_string(), "age".to_string()];
        let records = vec![
            vec!["giovani".to_string(), "22".to_string()],
            vec!["jubileu".to_string(), "100".to_string()],
        ];

        let map = pair_headers_with_records(&headers, &records);

        assert_eq!(map[0][0].0, "name");
        assert_eq!(map[0][0].1, "giovani");
        assert_eq!(map[0][1].0, "age");
        assert_eq!(map[0][1].1, "22");
    }

    #[test]
    fn should_serialize_single_object() {
        let input = vec![vec![
            ("giovani".to_string(), "beautiful".to_string()),
            ("age".to_string(), "22".to_string()),
        ]];

        let result = json_serialize(&input);

        assert_eq!(result, r#"[{"giovani": "beautiful", "age": 25}]"#);
    }

    #[test]
    fn should_serialize_multiple_objects() {
        let input = vec![
            vec![("giovani".to_string(), "beautiful".to_string())],
            vec![("age".to_string(), "22".to_string())],
        ];

        let result = json_serialize(&input);

        assert_eq!(result, r#"[{"giovani": "beautiful"}, {"age": 22}]"#);
    }

    #[test]
    fn should_not_quote_numbers() {
        let input = vec![vec![("age".to_string(), "22".to_string())]];

        let result = json_serialize(&input);

        assert_eq!(result, r#"[{"age": 22}]"#);
    }

    #[test]
    fn should_not_quote_booleans() {
        let input = vec![vec![("active".to_string(), "true".to_string())]];

        let result = json_serialize(&input);

        assert_eq!(result, r#"[{"active": true}]"#);
    }

    #[test]
    fn should_serialize_empty_object() {
        let input = vec![vec![]];

        let result = json_serialize(&input);

        assert_eq!(result, "[{}]");
    }

    #[test]
    fn should_return_empty_json_array_when_input_is_empty() {
        let input: Vec<Vec<(String, String)>> = vec![];

        let result = json_serialize(&input);

        assert_eq!(result, "[]");
    }

    #[test]
    fn concat_key_value_with_quoted_strings() {
        assert_eq!(
            concat_key_value("\"ball\"", "\"beautiful\""),
            "\"ball\": \"beautiful\""
        );
    }

    #[test]
    fn concat_key_value_with_empty_quoted_strings() {
        assert_eq!(concat_key_value("\"\"", "\"\""), "\"\": \"\"");
    }

    #[test]
    fn add_quotes_wraps_word() {
        assert_eq!(add_quotes("ball"), "\"ball\"");
    }

    #[test]
    fn add_quotes_wraps_empty_string() {
        assert_eq!(add_quotes(""), "\"\"");
    }

    #[test]
    fn add_brackets_wraps_word() {
        assert_eq!(add_brackets("ball"), "{ball}");
    }

    #[test]
    fn add_brackets_wraps_already_bracketed() {
        assert_eq!(add_brackets("{ball}"), "{{ball}}");
    }

    #[test]
    fn add_brackets_wraps_empty_string() {
        assert_eq!(add_brackets(""), "{}");
    }

    #[test]
    fn is_boolean_accepts_true_variants() {
        assert!(is_boolean("TRUE"));
        assert!(is_boolean("True"));
        assert!(is_boolean("true"));
    }

    #[test]
    fn is_boolean_accepts_false_variants() {
        assert!(is_boolean("FALSE"));
        assert!(is_boolean("False"));
        assert!(is_boolean("false"));
    }

    #[test]
    fn is_boolean_rejects_non_boolean() {
        assert!(!is_boolean("any_thing"));
    }

    #[test]
    fn is_numeric_accepts_comma_decimal() {
        assert!(is_numeric("19,90"));
    }

    #[test]
    fn is_numeric_accepts_dot_decimal() {
        assert!(is_numeric("19.90"));
    }

    #[test]
    fn is_numeric_rejects_dashes() {
        assert!(!is_numeric("19-992972393"));
    }

    #[test]
    fn is_numeric_rejects_scientific_notation() {
        assert!(!is_numeric("1290391e23"));
    }
}
