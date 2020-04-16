use std::io;
use reqwest;

fn main() -> reqwest::Result<()> {
    let body = reqwest::blocking::get("https://j2.business")?
      .text()?;
    println!("body = {:?}", body);
    Ok(())
  }