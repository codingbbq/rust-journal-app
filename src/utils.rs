use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::Local;
use once_cell::sync::Lazy;
use std::sync::Mutex;


// Global, lazily initiated journal path
static JOURNAL_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));

// Prompt the user for a file name 
fn ask_for_path() -> io::Result<PathBuf> {
    println!("Enter a journal name (or press <Enter> for default \"Journal.csv\"):");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    // Empty input -> default name
    let file_name = if trimmed.is_empty() {
        "Journal.csv".to_string()
    } else {
        trimmed.to_string()
    };

    Ok(PathBuf::from(file_name))
}

// Public accessor - return the path to the CSV file, creating it if necessary.
pub fn journal_path() -> io::Result<PathBuf> {
    let mut guard = JOURNAL_PATH.lock().unwrap();
    if let Some(ref cached) = *guard {
        return Ok(cached.clone());
    }

    // Look for an existing *.csv file in the current directory
    let cwd = std::env::current_dir()?;
    let existing_csv = std::fs::read_dir(&cwd)?
        .filter_map(Result::ok)
        .find(|e| {
            e.path()
             .extension()
             .map_or(false, |ext| ext.eq_ignore_ascii_case("csv"))
        })
        .map(|e| e.path());

    // If we found one, use it, otherwise ask the user
    let path = match existing_csv {
        Some(p) => p,
        None => ask_for_path()?,
    };

    // Cache for the rest of the program run
    *guard = Some(path.clone());

    Ok(path)
}

/// Ensure the CSV file exists and write the header if it does not.
pub fn ensure_journal_file() -> io::Result<()> {
    let path = journal_path()?;
    if !path.exists() {
        // Create the file and write the header
        let mut file = File::create(&path)?;
        file.write_all(b"Timestamp,Journal Entry,Tags\n")?;
        println!("Created {} file", path.display());
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

/// Extract all tags from a string (words starting with #).
pub fn extract_tags(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.starts_with('#'))
        .map(|w| w.trim_start_matches('#').to_string())
        .collect()
}