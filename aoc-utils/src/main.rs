use std::ops::RangeInclusive;
use std::{fs, path::Path};

use chrono::{Datelike, FixedOffset, Utc};
use clap::Parser;
use dotenv::dotenv;
use reqwest::{
    blocking::ClientBuilder,
    header::{COOKIE, USER_AGENT},
};

const DEFAULT_INPUT_FOLDER: &str = "./input";
const DAY_RANGE: RangeInclusive<u32> = 1..=25;

/// Utility for Advent of Code that downloads the puzzle input for a given year and day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year to download puzzle input for.
    /// Defaults to current year.
    #[arg(short, long, default_value_t=get_current_year())]
    year: i32,

    /// Day to download puzzle input for.
    /// Defaults to current day.
    #[arg(short, long, value_parser=day_in_range, default_value_t=get_current_day())]
    day: u32,

    /// Location to save puzzle input files.
    /// Defaults to `./input`.
    #[arg(short, long, default_value = DEFAULT_INPUT_FOLDER)]
    input_folder: String,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    println!("Day {}", args.day);
    // TODO: Project/day setup stuff, like .gitignore the `input` folder,
    // make a new file for the code of a new day, etc.

    // TODO: Download all released days
    let year = args.year;
    let day = args.day;
    let input_folder = args.input_folder;

    if !Path::new(&get_puzzle_input_path(&input_folder, day)).exists() {
        println!("Downloading puzzle input for Day {}, {}", day, year);
        let puzzle_input = download_puzzle_input(year, day)?;
        save_puzzle_input(&input_folder, day, puzzle_input)?;
    } else {
        println!("Puzzle input already downloaded for Day {}, {}", day, year);
    }

    Ok(())
}

fn day_in_range(s: &str) -> Result<u32, String> {
    let day: u32 = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
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

fn download_puzzle_input(year: i32, day: u32) -> Result<String, Box<dyn std::error::Error>> {
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
        return Err(format!(
            "Failed to download puzzle input: {}",
            puzzle_input_response.status()
        )
        .into());
    }

    let puzzle_input = puzzle_input_response.text()?;

    Ok(puzzle_input)
}

fn save_puzzle_input(
    input_folder: &String,
    day: u32,
    puzzle_input: String,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(input_folder)?;
    fs::write(get_puzzle_input_path(input_folder, day), puzzle_input)
        .expect("File could not be saved");

    Ok(())
}

fn get_puzzle_input_path(input_folder: &String, day: u32) -> String {
    format!("{}/day{}.txt", input_folder, day)
}

fn get_current_day() -> u32 {
    // Advent of Code releases puzzles at midnight EST
    let offset = FixedOffset::east_opt(3600 * -5).unwrap();
    Utc::now().with_timezone(&offset).day()
}

fn get_current_year() -> i32 {
    // Advent of Code releases puzzles at midnight EST
    let offset = FixedOffset::east_opt(3600 * -5).unwrap();
    Utc::now().with_timezone(&offset).year()
}
