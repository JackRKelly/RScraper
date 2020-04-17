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
    //println!("body = {:?}", body);

    let link_regex = Regex::new(r#"href=(?:"([^"]+)"|'(?:'([^']+)'))"#).unwrap();

    for links in link_regex.captures_iter(&body) {
        let current_link = links.get(1).unwrap().as_str();
        if current_link.starts_with("http://") || current_link.starts_with("https://") {
            println!("{}", current_link);
        } else if current_link.starts_with("#") {

        } else {
            println!("{}/{}", link.trim(), current_link);
        }
    }

    Ok(())
}
