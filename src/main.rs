use std::fs::OpenOptions;
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

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)
        {
            Ok(_) => {
                println!("Journey.csv created");
                Ok(())
            }
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                println!("Joureny.csv already exists");
                Ok(())
            }

            Err(err) => Err(err),
        }
}