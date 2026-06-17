# CSV Manager

A simple CLI tool for viewing, filtering, and exporting CSV data.

### Show CSV Data

Display the contents of a CSV file in the terminal.

```bash
cargo run <path.csv> show
```

### Filter Rows

Filter rows based on a search term.

```bash
cargo run <path.csv> filter "<search-term>"
```

Example:

```bash
cargo run users.csv filter "Price>2000"
```

### Export to JSON

Convert a CSV file to JSON and print the result to the terminal.

```bash
cargo run <path.csv> export
```
