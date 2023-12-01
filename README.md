# Advent of Code
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge that encourages people to solve problems using programming. This repository holds any attempts I make at completing the challenge!

# Years
## 2020, 2021, 2022, 2023: Rust
I decided to do the 2020, 2021, 2022, 2023 challenge using Rust. I don't use Rust professionally, so I want to flex my problem solving skills with it using Advent of Code.

### Running the Project
- Install the Rust tooling
- Navigate to `rust-202#/`
- `cargo run` will run a CLI program that outputs info for each of the challenges
- `cargo test` will run all the unit tests

### How to Write Days
- TODO: Explain the basics

### Needed Updates
- I moved from an `Option` to a `Result` on the `Day` response, but haven't migrated 2020 or 2021 years over yet.
- The `inputs` folder shouldn't be pushed to GitHub to respect the wishes of the AoC creators. I need to add a script to download the inputs for each day, and clear my inputs for all years before 2023
- Test macro should be able to take in two inputs, sometimes the example puzzle input changes on part 2!
