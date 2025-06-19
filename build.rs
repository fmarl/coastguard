use std::fs;
use std::io::Read;
use std::path::Path;

fn generate_tlds() {
    // Download the IANA TLD list
    let mut resp = reqwest::blocking::get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
        .expect("Failed to download TLD list");
    let mut body = String::new();
    resp.read_to_string(&mut body).expect("Read error");

    // Filter out comments and blank lines
    let tlds: Vec<String> = body
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|s| s.to_lowercase())
        .collect();

    let out = format!(
        "pub static TLDS: &[&str] = &{:?};",
        tlds.iter().map(String::as_str).collect::<Vec<&str>>()
    );
    
    fs::create_dir_all(Path::new(&std::env::var("OUT_DIR").unwrap())).unwrap();
    fs::write(Path::new(&std::env::var("OUT_DIR").unwrap()).join("tlds.rs"), out).unwrap();
}

fn main() {
    generate_tlds();
}