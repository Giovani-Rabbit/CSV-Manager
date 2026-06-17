use colored::Colorize;

pub fn print_table(headers: &[String], records: &[Vec<String>], limit: Option<usize>) {
    let spacing = column_widths(headers, records);
    let formatted_headers = header_spacing(headers, &spacing);

    let output = [
        format_header(&formatted_headers),
        format_line_separetor(&spacing),
        format_records(&spacing, records, limit),
    ];

    println!("{}", output.join("\n"));
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

fn format_header(headers: &[String]) -> String {
    headers.join(" | ").bold().cyan().to_string()
}

fn format_line_separetor(spacing: &[usize]) -> String {
    spacing
        .iter()
        .map(|l| "-".repeat(*l))
        .collect::<Vec<String>>()
        .join("-+-")
}

fn format_records(spacing: &[usize], records: &[Vec<String>], limit: Option<usize>) -> String {
    let effective_limit = limit.unwrap_or(20);
    let mut columns: Vec<String> = Vec::with_capacity(effective_limit);

    for record in records.iter().take(effective_limit) {
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

        columns.push(column.join(" | "));
    }

    columns.join("\n")
}
