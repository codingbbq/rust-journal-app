use std::env;
use std::fs::{File, OpenOptions};
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

fn print_help() {
    println!("Usage:");
    println!(" Cargo run -- add \"your journal entry\"");
}