use std::ops::RangeInclusive;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, path::Path};

use clap::Parser;
use dotenv::dotenv;
use reqwest::{
    blocking::ClientBuilder,
    header::{COOKIE, USER_AGENT},
};

const INPUT_FOLDER: &str = "./input";
const DAY_RANGE: RangeInclusive<isize> = 1..=25;

/// Utility for Advent of Code that downloads the puzzle input for a given year and day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Year to download puzzle input for, defaults to current year
    #[arg(short, long)]
    year: Option<isize>,

    /// Day to download puzzle input for
    #[arg(short, long, value_parser=day_in_range)]
    day: isize,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    println!("Day {}", args.day);
    // TODO: Project/day setup stuff, like .gitignore the `input` folder,
    // make a new file for the code of a new day, etc.

    // TODO: Default to current day if not provided
    // TODO: Can I do the get_current_year default with Clap?
    // TODO: Download all released days
    let year = args.year.unwrap_or(get_current_year());
    let day = args.day;

    if !Path::new(&get_puzzle_input_path(day)).exists() {
        println!("Downloading puzzle input for Day {}, {}", day, year);
        let puzzle_input = download_puzzle_input(year, day)?;
        save_puzzle_input(day, puzzle_input)?;
    } else {
        println!("Puzzle input already downloaded for Day {}, {}", day, year);
    }

    Ok(())
}

fn day_in_range(s: &str) -> Result<isize, String> {
    let day: isize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
    if DAY_RANGE.contains(&day) {
        Ok(day)
    } else {
        Err(format!(
            "Day not in range {}-{}",
            DAY_RANGE.start(),
            DAY_RANGE.end()
        ))
    }
}

fn download_puzzle_input(year: isize, day: isize) -> Result<String, Box<dyn std::error::Error>> {
    let token: String = std::env::var("AOC_AUTH_TOKEN")
        .expect("AOC_AUTH_TOKEN must be set. Get it from your browser's cookies!");

    let client = ClientBuilder::new().build()?;

    let puzzle_input_response = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        // TODO: Use proper value for user agent (should be contact info of user, get from Cargo.toml or something)
        .header(USER_AGENT, "lanesawyer/aoc-utils")
        .header(COOKIE, format!("session={}", token))
        .send()?;

    if reqwest::StatusCode::NOT_FOUND == puzzle_input_response.status() {
        return Err(format!("Day {} is not available yet", day).into());
    } else if !puzzle_input_response.status().is_success() {
        return Err(format!("Failed to download puzzle input: {}", puzzle_input_response.status()).into());
    }

    let puzzle_input = puzzle_input_response.text()?;
    
    Ok(puzzle_input)
}

fn save_puzzle_input(day: isize, puzzle_input: String) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(INPUT_FOLDER)?;
    fs::write(get_puzzle_input_path(day), puzzle_input).expect("File could not be saved");

    Ok(())
}

fn get_puzzle_input_path(day: isize) -> String {
    format!("{}/day{}.txt", INPUT_FOLDER, day)
}

fn get_current_year() -> isize {
    // Get the current duration since the UNIX epoch
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time");

    // Calculate the current year
    1970 + current_time.as_secs() as isize / (365 * 24 * 60 * 60)
}
