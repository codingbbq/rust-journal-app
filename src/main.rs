use std::env;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(()) => {}
        Err(err) => eprintln!("Error : {}", err),
    }
}

fn run(args: Vec<String>) -> io::Result<()> {
    ensure_journal_file()?;

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let command = &args[1];

    if command == "add" {
        if args.len() < 3 {
            eprintln!("Please provide journal text");
            eprintln!("Example: cargo run -- add \"Today I learned Rust basics\"");
            return Ok(());
        }

        let entry_text = args[2..].join(" ");
        append_entry(&entry_text)?;
        println!("Entry saved");
    } else if command == "list" {
        list_entries()?;
    } else if command == "search" {
        if args.len() < 3 {
            eprintln!("Please provide a search keyword");
            eprintln!("Example: cargo run -- search rust");
            return Ok(());
        }

        let query = args[2..].join(" ");
        search_entries(&query)?;
    } else if command == "stats" {
        show_stats()?;
    } else {
        eprintln!("Unknown command: {}", command);
        print_help();
    }

    Ok(())
}

fn ensure_journal_file() -> io::Result<()> {
    let file_path = Path::new("Journal.csv");

    if !file_path.exists() {
        let mut file = File::create(file_path)?;
        file.write_all(b"Timestamp,Journal Entry\n")?;
        println!("Journal.csv created with header");
    }

    Ok(())
}

fn append_entry(entry_text: &str) -> io::Result<()> {
    let timestamp = get_timestamp();

    let mut file = OpenOptions::new()
    .append(true)
    .open("Journal.csv")?;

    writeln!(file, "{},{}", timestamp, format_csv_field(entry_text))?;
    Ok(())
}

fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn format_csv_field(input: &str) -> String {
    format!("\"{}\"", escape_csv(input))
}

fn escape_csv(input: &str) -> String {
    input.replace('\"', "\"\"")
}

fn list_entries() -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();

    lines.next(); // Skip the header

    let entries: Vec<&str> = lines.collect();

    if entries.is_empty() {
        println!("No journal entries yet");
        return Ok(());
    }

    println!("\n--- Journal Entries ---");
    for(index, line) in entries.iter().enumerate() {
        println!("[{}] {}", index+1, line);
    }

    println!("--- {} entries --- \n", entries.len());

    Ok(())
}

// For Search Command
fn search_entries(query: &str) -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();

    lines.next(); // Skip the header

    let query_lower = query.to_lowercase();
    let mut found = 0;

    println!("\n--- Search Results for : {} ---", query);

    for line in lines {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&query_lower) {
            found+= 1;
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

fn show_stats() -> io::Result<()> {
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();

    lines.next(); // skip the header

    let entries: Vec<&str> = lines.collect();
    let total = entries.len();

    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut today_count = 0;

    for line in &entries {
        if line.starts_with(&today) {
            today_count += 1;
        }
    }

    println!("\n--- Journal Stats ---");
    println!("Total entries : {}", total);
    println!("Entries today ({}): {}", today, today_count);
    println!("-----------------\n");

    Ok(())
}

fn print_help() {
    println!("Usage:");
    println!(" Cargo run -- add \"your journal entry\"");
    println!(" Cargo run -- list");
    println!(" Cargo run -- search rust");
    println!(" Cargo run -- stats");
}