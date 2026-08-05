use std::fs::read_to_string;
use std::io;
use crate::utils;

/// List all entries.
pub fn list_entries() -> io::Result<()> {
    let path = utils::journal_path()?;

    let content = read_to_string(path)?;
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