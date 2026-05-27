use colored::Colorize;

pub fn print_table(headers: &[String], records: &[Vec<String>], limit: usize) {
    let spacing = column_widths(headers, records);
    let formatted_headers = header_spacing(headers, &spacing);

    print_header(&formatted_headers);
    print_line_separetor(&spacing);
    print_records(&spacing, records, limit)
}

fn column_widths(headers: &[String], records: &[Vec<String>]) -> Vec<usize> {
    let mut spaces: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for line in records {
        for (idx, column) in line.iter().enumerate() {
            if idx < headers.len() && column.len() > spaces[idx] {
                spaces[idx] = column.len();
            }
        }
    }

    return spaces;
}

fn header_spacing(headers: &[String], spacing: &[usize]) -> Vec<String> {
    headers
        .iter()
        .enumerate()
        .map(|(idx, header)| {
            format!(
                "{:space_size$}",
                header,
                space_size = spacing.get(idx).copied().unwrap_or(10)
            )
        })
        .collect()
}

fn print_header(headers: &[String]) {
    println!("{}", headers.join(" | ").bold().cyan())
}

fn print_line_separetor(spacing: &[usize]) {
    let table_line_separator: Vec<String> = spacing.iter().map(|l| "-".repeat(*l)).collect();
    println!("{}", table_line_separator.join("-+-"))
}

fn print_records(spacing: &[usize], records: &[Vec<String>], limit: usize) {
    for record in records.iter().take(limit) {
        let column: Vec<String> = record
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                let column_len = c.len();
                let space = spacing.get(idx).unwrap_or(&column_len);
                let truncated = if c.len() > *space {
                    format!("{}...", &c[..space - 3])
                } else {
                    c.clone()
                };
                format!("{:space$}", truncated, space = space)
            })
            .collect();
        println!("{}", column.join(" | "))
    }
}
