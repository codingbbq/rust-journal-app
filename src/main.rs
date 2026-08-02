use std::env;
use std::io;
mod utils;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(()) => {}
        Err(err) => eprintln!("Error : {}", err),
    }
}

fn run(args: Vec<String>) -> io::Result<()> {
    utils::ensure_journal_file()?;

    if args.len() < 2 {
        commands::print_help();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
                if args.len() < 3 {
                eprintln!("Please provide journal text");
                eprintln!("Example: cargo run -- add \"Today I learned Rust basics\"");
                return Ok(());
            }

            let entry_text = args[2..].join(" ");
            commands::append_entry(&entry_text)?;
            println!("Entry saved");
        },
        "list" => commands::list_entries()?,
        "search" => {
            if args.len() < 3 {
                eprintln!("Please provide a search keyword");
                eprintln!("Example: cargo run -- search rust");
                return Ok(());
            }

            let query = args[2..].join(" ");
            commands::search_entries(&query)?;    
        },
        "stats" => commands::show_stats()?,
        _ => {
            eprintln!("Unknonwn command: {}", command);
            commands::print_help();
        }
    }

    Ok(())
}