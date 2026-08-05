// On the user added entries, do a filter on tags i. e strings start with #
use std::io;
use std::fs::read_to_string;

pub fn filter_entries(tag: &str) -> io::Result<()> {
    // Read the whole CSV file
    let content = read_to_string("Journal.csv")?;
    let mut lines = content.lines();

    // The first line is a header
    if let Some(header) = lines.next() {
        println!("--- {} (filtered by #{})---", header, tag);
    }

    for line in lines {
        let mut parts = line.splitn(3, ",");
        let timestamp = parts.next().unwrap_or("");
        let entry = parts.next().unwrap_or("");

        let tags_field = parts.next().unwrap_or("");

        let clean_tags = tags_field.trim_matches('"');
        let tag_list: Vec<&str> = clean_tags.split(',').collect();

        if tag_list.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
            println!("{} | {} | {}", timestamp, entry, tags_field);
        }
    }

    Ok(())
}