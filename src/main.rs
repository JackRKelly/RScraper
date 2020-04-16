use std::io;
use reqwest;
use regex::Regex;

fn main() -> reqwest::Result<()> {
    println!("Please input your link.");

    let mut link = String::new();

    io::stdin().read_line(&mut link)
        .expect("Failed to read line");

    let body = reqwest::blocking::get(&link)?
      .text()?;
    //println!("body = {:?}", body);

    let link_regex = Regex::new(r#"href=(?:"([^"]+)"|'(?:'([^']+)'))"#).unwrap();

    for link in link_regex.captures_iter(&body) {
        println!("{:?}", link.get(1).unwrap().as_str());
    }


    Ok(())
}