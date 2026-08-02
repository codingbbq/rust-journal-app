use std::fs::OpenOptions;
use std::io::{self, Write};
use std::fs::read_to_string;
use chrono::Local;

use crate::utils;

/// Append a new entry to the CSV.
pub fn append_entry(entry_text: &str) -> io::Result<()> {
    let timestamp = utils::get_timestamp();
    let mut file = OpenOptions::new()
        .append(true)
        .open("Journal.csv")?;
    writeln!(file, "{},{}", timestamp, utils::format_csv_field(entry_text))?;
    Ok(())
}

/// List all entries.
pub fn list_entries() -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();
    lines.next(); // skip header
    let entries: Vec<&str> = lines.collect();

    if entries.is_empty() {
        println!("No journal entries yet");
        return Ok(());
    }

    println!("\n--- Journal Entries ---");
    for (i, line) in entries.iter().enumerate() {
        println!("[{}] {}", i + 1, line);
    }
    println!("--- {} entries ---\n", entries.len());
    Ok(())
}

/// Search for a keyword.
pub fn search_entries(query: &str) -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();
    lines.next(); // skip header
    let query_lower = query.to_lowercase();
    let mut found = 0;

    println!("\n--- Search Results for: {} ---", query);
    for line in lines {
        if line.to_lowercase().contains(&query_lower) {
            found += 1;
            println!("[{}] {}", found, line);
        }
    }

    if found == 0 {
        println!("No matching entries found");
    } else {
        println!("--- {} match(es) ---", found);
    }
    println!();
    Ok(())
}

/// Show simple statistics.
pub fn show_stats() -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();
    lines.next(); // skip header
    let entries: Vec<&str> = lines.collect();
    let total = entries.len();

    let today = Local::now().format("%Y-%m-%d").to_string();
    let today_count = entries.iter().filter(|l| l.starts_with(&today)).count();

    println!("\n--- Journal Stats ---");
    println!("Total entries : {}", total);
    println!("Entries today ({}): {}", today, today_count);
    println!("-----------------\n");
    Ok(())
}

/// Print usage help.
pub fn print_help() {
    println!("Usage:");
    println!(" cargo run -- add \"your journal entry\"");
    println!(" cargo run -- list");
    println!(" cargo run -- search <keyword>");
    println!(" cargo run -- stats");
}