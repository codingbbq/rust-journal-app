use std::io;
use std::fs::read_to_string;
use crate::utils;

/// Search for a keyword.
pub fn search_entries(query: &str) -> io::Result<()> {
    let path = utils::journal_path()?;
    let content = read_to_string(path)?;
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