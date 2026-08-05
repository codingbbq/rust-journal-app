// src/commands/add.rs
use std::fs::OpenOptions;
use std::io::{self, Write};
use crate::utils;
 
pub fn append_entry(entry_text: &str) -> io::Result<()> {
    let timestamp = utils::get_timestamp();

    // Collect tags from the entry text
    let tags = utils::extract_tags(entry_text);

    let tag_field = tags.join(",");

    let path = utils::journal_path()?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(
        file,
        "{},{},{}", 
        timestamp, 
        utils::format_csv_field(entry_text),
        utils::format_csv_field(&tag_field)
    )?;
    Ok(())
}