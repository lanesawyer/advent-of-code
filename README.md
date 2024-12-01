# Advent of Code
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge that encourages people to solve problems using programming. This repository holds any attempts I make at completing the challenge!

# Years
## 2020, 2021, 2022, 2023, 2024: Rust
I decided to do the 2020, 2021, 2022, 2023, and 2024 challenge using Rust. I don't use Rust professionally, so I want to flex my problem solving skills with it using Advent of Code.

### Running the Project
- Install the Rust tooling
- Navigate to `rust-202#/` for whatever year you want to run
- `cargo run` will run a CLI program that outputs info for each of the challenges
- `cargo test` will run all the unit tests

### How to Write Days
- TODO: Explain the basics

### Needed Updates
#### aoc-utils
- More helper functions for common input parsing needs
- Run a single day instead of all of them
- Instructions on how to use the CLI tooling
- Submission of answers (and rate limiting so I don't accidentally spam the server)
- Publish helper crates to crates.io
- More CLI options (like setting the `input` storage directory)
- Festive ASCII art

#### Problem Solving Code
- Remove deprecated read_input function in 2021's code
- Set up a workspace so I can have all years depend on workspace dependencies, which makes me feel better about pulling in a variety of helpful crates
