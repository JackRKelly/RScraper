use regex::Regex;
use reqwest;
use std::io;

fn main() -> reqwest::Result<()> {
    println!("Please input your link.");

    let mut link = String::new();
    let mut link_list: Vec<String> = Vec::new();

    io::stdin()
        .read_line(&mut link)
        .expect("Failed to read line");

    let body = reqwest::blocking::get(&link)?.text()?;
    let link_regex = Regex::new(r#"href=(?:"([^"]+)"|'(?:'([^']+)'))"#).unwrap();

    for links in link_regex.captures_iter(&body) {
        let current_link = links.get(1).unwrap().as_str();
        if current_link.starts_with("http://") || current_link.starts_with("https://") {
            link_list.push(current_link.to_string());
        } else if current_link.starts_with('#') {
        } else {
            link_list.push(format!("{}/{}", link.trim(), current_link));
        };
    }

    print_table(link, link_list);
    Ok(())
}

fn get_padding(s: String, length: usize) -> String {
    let padding = length - s.len();
    vec![' '; padding].iter().collect::<String>()
}

fn print_table(title: String, list: Vec<String>) {
    let max = list.iter().map(|s| s.len()).max();
    let separator = vec!['-'; max.unwrap()].iter().collect::<String>();
    println!("+-{}-+", separator);
    println!(
        "| {}{} |",
        title.trim(),
        get_padding(title.trim().to_string(), max.unwrap())
    );
    println!("+-{}-+", separator);
    for lines in list.iter() {
        println!(
            "| {}{} |",
            lines,
            get_padding(lines.to_string(), max.unwrap())
        );
    }
    println!("+-{}-+", separator);
}
