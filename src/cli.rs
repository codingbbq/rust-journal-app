use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="rust_journal_app")]
#[command(author="codingbbq <codingbbq@gmail.com>")]
#[command(version="1.0.0")]
#[command(about="A tiny Jornal CLI")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// All sub-commands the app supports
#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(required = true)]
        entry: Vec<String>,
    },

    List,

    Search {
        #[arg(required = true)]
        query: Vec<String>,
    },

    Stats,

    Filter {
        #[arg(required = true)]
        tag: Vec<String>,
    },

    Manual,
}