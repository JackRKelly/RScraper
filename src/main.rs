use std::io;

fn main() {
    println!("Input link to be scraped.");

    let mut link = String::new();

    io::stdin().read_line(&mut link)
    .expect("Failed to read line");


    println!("Scraping: {}", link)
}
