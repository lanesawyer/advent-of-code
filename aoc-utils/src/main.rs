use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, path::Path};

use dotenv::dotenv;
use reqwest::{
    blocking::ClientBuilder,
    header::{COOKIE, USER_AGENT},
};

const INPUT_FOLDER : &str = "./input";

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Get the current duration since the UNIX epoch
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time");

    // Calculate the current year
    let current_year = 1970 + current_time.as_secs() as i64 / (365 * 24 * 60 * 60);
    let day = 1;

    if !Path::new(&get_puzzle_input_path(day)).exists() {
        println!("Downloading puzzle input for Day {}, {}", 1, current_year);
        let puzzle_input = download_puzzle_input(current_year.try_into()?, day.try_into()?)?;
        save_puzzle_input(1, puzzle_input)?;
    } else {
        println!(
            "Puzzle input already downloaded for Day {}, {}",
            1, current_year
        );
    }

    Ok(())
}

fn download_puzzle_input(year: isize, day: isize) -> Result<String, Box<dyn std::error::Error>> {
    let token: String = std::env::var("AOC_AUTH_TOKEN")
        .expect("AOC_AUTH_TOKEN must be set. Get it from your browser's cookies!");

    let client = ClientBuilder::new().build()?;

    let puzzle_input = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        // TODO: Use proper value for user agent (should be contact info of user, get from Cargo.toml or something)
        .header(USER_AGENT, "lanesawyer/aoc-utils")
        .header(COOKIE, format!("session={}", token))
        .send()?
        .text()?;

    Ok(puzzle_input)
}

fn save_puzzle_input(day: usize, puzzle_input: String) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(INPUT_FOLDER)?;
    fs::write(
        get_puzzle_input_path(day),
        puzzle_input,
    )
    .expect("File could not be saved");

    Ok(())
}

fn get_puzzle_input_path(day: usize) -> String {
    format!("{}/day{}.txt", INPUT_FOLDER, day)
}
