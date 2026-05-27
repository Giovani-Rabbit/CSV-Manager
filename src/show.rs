use colored::Colorize;

pub fn table(headers: &[String], lines: &[Vec<String>], limit: usize) {
    let mut width: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for line in lines.iter().take(limit) {
        for (i, campo) in line.iter().enumerate() {
            if i < width.len() && campo.len() > width[i] {
                width[i] = campo.len();
            }
        }
    }

    for l in width.iter_mut() {
        if *l > 40 {
            *l = 40;
        }
    }

    let table_headers: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = width.get(i).copied().unwrap_or(10)))
        .collect();
    println!("{}", table_headers.join(" | ").bold().cyan());

    let table_line_separator: Vec<String> = width.iter().map(|l| "-".repeat(*l)).collect();
    println!("{}", table_line_separator.join("-+-"));

    // Linhas de dados
    let total_lines = lines.len();
    for line in lines.iter().take(limit) {
        let campos: Vec<String> = line
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let largura = width.get(i).copied().unwrap_or(10);
                let truncado = if c.len() > largura {
                    format!("{}...", &c[..largura - 3])
                } else {
                    c.clone()
                };
                format!("{:width$}", truncado, width = largura)
            })
            .collect();
        println!("{}", campos.join(" | "));
    }

    if total_lines > limit {
        println!(
            "\n{} Showind {} of {} records.",
            "INFO:".blue().bold(),
            limit,
            total_lines
        );
    }
}
