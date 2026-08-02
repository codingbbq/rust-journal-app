use std::io;
use std::fs::read_to_string;

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