use std::fs::write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/"; 
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown..."); 
    let md = html2md::parse_html(&body);

    write(output, md.as_bytes()).unwrap(); 
    println!("Converted markdown has been saved in {}.", output);
}