use core::panic;
use std::{
    borrow::Borrow,
    fs::{create_dir, read_to_string, File},
    io::Write,
    process::Command,
};

use chrono::Datelike;
use clap::Parser;
use reqwest::Url;
use tokio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // path: String,
    #[arg(short, long, default_value_t = chrono::Utc::now().year())]
    year: i32,

    #[arg(short, long, default_value_t = chrono::Utc::now().day())]
    day: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let year = args.year;
    let day = args.day;

    let url =
        Url::parse(format!("https://adventofcode.com/{year}/day/{day}/input").borrow()).unwrap();

    let client = reqwest::Client::new();

    let session_key = match read_to_string("session_key.txt") {
        Ok(session_key) => session_key,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    if session_key.is_empty() {
        panic!("Session key is empty");
    }

    let response = client
        .get(url)
        .header("Cookie", format!("session={session_key}"))
        .header(
            "User-Agent",
            "https://github.com/username2000w/AoC-Rust-Utils/ by username2000w",
        )
        .send()
        .await?;

    let input = response.text().await?;

    match Command::new("cargo")
        .arg("new")
        .arg(format!("day{day}"))
        .spawn()
    {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => println!("failed to execute process: {}", e),
    }

    let mut file = File::create(format!("day{day}/input.txt"))?;
    file.write_all(input.as_bytes())?;

    create_dir(format!("day{day}/src/bin"))?;

    let mut file = File::create(format!("day{day}/src/bin/part1.rs"))?;
    file.write_all(b"fn main() {}\n".as_slice())?;

    let mut file = File::create(format!("day{day}/src/bin/part2.rs"))?;
    file.write_all(b"fn main() {}\n".as_slice())?;

    Ok(())
}
