use std::io;
use clap::Parser;
mod utils;
mod commands;
mod cli;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("Error: {}", err);
    }
}

fn run(cli: cli::Cli) -> io::Result<()> {
    utils::ensure_journal_file()?;

    utils::ensure_journal_file()?;
    
    match cli.command {
        cli::Commands::Add { entry } => {
            let entry_text = entry.join(" ");
            commands::append_entry(&entry_text)?;
            println!("Entry saved");
        },
        cli::Commands::List => {
            commands::list_entries()?;
        }
        cli::Commands::Search { query } => {
            let query = query.join(" ");
            commands::search_entries(&query)?;
        },
        cli::Commands::Stats => {
            commands::show_stats()?;
        } 
        cli::Commands::Filter { tag } => {
            let tag = tag.join(" ");
            commands::filter_entries(&tag)?;
        },
        cli::Commands::Manual => {
            commands::print_manual();
        }
    }

    Ok(())
}