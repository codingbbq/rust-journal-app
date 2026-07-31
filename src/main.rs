use std::fs::File;
use std::io;
use std::path::Path;

fn main() {
    match ensure_journal_file() {
        Ok(()) => println!("Journey.csv is ready."),
        Err(err) => eprintln!("Failed to prepare Journey.csv : {}", err),
    }
}

fn ensure_journal_file() -> io::Result<()> {
    let file_path = Path::new("Journey.csv");

    if !file_path.exists() {
        File::create(file_path)?;
        println!("Journey.csv created");
    } else {
        println!("Journey.csv already exists.");
    }

    Ok(())
}