use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

/// Ensure the CSV file exists and write the header if it does not.
pub fn ensure_journal_file() -> io::Result<()> {
    let file_path = Path::new("Journal.csv");
    if !file_path.exists() {
        let mut file = File::create(file_path)?;
        file.write_all(b"Timestamp,Journal Entry\n")?;
        println!("Journal.csv created with header");
    }
    Ok(())
}

/// Return the current timestamp as a formatted string.
pub fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Escape a CSV field and wrap it in quotes.
pub fn format_csv_field(input: &str) -> String {
    format!("\"{}\"", escape_csv(input))
}

/// Double‑quote any `"` characters so the CSV stays valid.
pub fn escape_csv(input: &str) -> String {
    input.replace('\"', "\"\"")
}