use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    if command == "add" {
        if args.len() < 3 {
            eprintln!("Please provide a journal entry.");
            eprintln!("Example: cargo run -- add \"Today was a good day\"");
            return;
        }

        let entry = args[2..].join(" ");
        println!("Journal entry captured: {}", entry);
    } else {
        eprintln!("Unknown command: {}", command);
        print_help();
    }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run -- add \"your journal entry\"");
}