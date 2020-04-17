use prettytable::{Table, Row, Cell};
use regex::Regex;
use reqwest;
use std::io;

fn main() -> reqwest::Result<()> {
    println!("Please input your link.");

    let mut link = String::new();

    io::stdin()
        .read_line(&mut link)
        .expect("Failed to read line");

    let body = reqwest::blocking::get(&link)?.text()?;
    let link_regex = Regex::new(r#"href=(?:"([^"]+)"|'(?:'([^']+)'))"#).unwrap();
    let mut table = Table::new();
    
    table.add_row(Row::new(vec![Cell::new(&format!("Scrape results for: {}", link))]));

    for links in link_regex.captures_iter(&body) {
        let current_link = links.get(1).unwrap().as_str();
        if current_link.starts_with("http://") || current_link.starts_with("https://") {
            table.add_row(Row::new(vec![Cell::new(current_link)]));
        } else if current_link.starts_with("#") {

        } else {
            table.add_row(Row::new(vec![Cell::new(&format!("{}/{}", link.trim(), current_link))]));
        };
    }
    table.printstd();

    Ok(())
}
