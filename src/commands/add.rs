// src/commands/add.rs
use std::fs::OpenOptions;
use std::io::{self, Write};
use crate::utils;
 
pub fn append_entry(entry_text: &str) -> io::Result<()> {
    let timestamp = utils::get_timestamp();
    let mut file = OpenOptions::new()
        .append(true)
        .open("Journal.csv")?;
    writeln!(file, "{},{}", timestamp, utils::format_csv_field(entry_text))?;
    Ok(())
}