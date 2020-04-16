use std::io;
use reqwest;

fn main() -> reqwest::Result<()> {
    println!("Please input your link.");

    let mut link = String::new();

    io::stdin().read_line(&mut link)
        .expect("Failed to read line");

    let body = reqwest::blocking::get(&link)?
      .text()?;
    println!("body = {:?}", body);
    Ok(())
}