use std::fs::read_to_string;
use std::io;
use chrono::Local;

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