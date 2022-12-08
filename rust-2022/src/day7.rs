use std::collections::HashMap;

use aoc_utils::{test_day, AdventError, Answer, Day};

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    children: Vec<&'a Directory<'a>>,
    files: Vec<File>,
    parent: Option<&'a Directory<'a>>,
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

pub struct Day7;

impl Day for Day7 {
    // Not sure what's wrong. Might work better to simply build a tree instead of using the Map
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

        let size_limit = 100_000;
        let mut total_size = 0;
        let root_directory = Directory {
                name: "/".to_string(),
                files: Vec::new(),
                children: Vec::new(),
                parent: None,
            };
        let mut current_directory = &mut root_directory;

        for line in lines {
            let mut commands = line.split_whitespace();
            let first_command = commands.next().unwrap();

            if first_command == "$" {
                let command = commands.next().unwrap();
                match command {
                    "ls" => println!("listing items"),
                    "cd" => {
                        let next_directory = commands.next().unwrap().to_string();

                        if next_directory == ".." {
                            let directory = current_directory;
                            current_directory = &mut directory.parent.unwrap();
                            println!("going up to {:#?}", current_directory);
                        }
                        else {
                            let next_dir: Vec<&Directory> = current_directory.children.into_iter().filter(|dir| dir.name == next_directory).collect();
                            let mut current = *next_dir.first().unwrap();
                            current_directory = &mut current;
                            println!("changing from to {:#?}", current_directory);
                        }
                    }
                    _ => unreachable!("Unknown command"),
                }
            } else if first_command == "dir" {
                let directory_name = commands.next().unwrap().to_string();

                // CHECK: Does this overwrite any existing data if we ran into this directory before
                    let new_directory = Directory {
                        name: directory_name.to_string(),
                        files: Vec::new(),
                        children: Vec::new(),
                        parent: Some(&current_directory),
                    };
                    current_directory
                        .children
                        .push(&new_directory);
                    println!("{:#?}", current_directory);

                println!("found directory {}", directory_name);
            } else if first_command.parse::<usize>().is_ok() {
                let file_size = first_command;
                let file_name = commands.next().unwrap();

                let file = File {
                    _name: file_name.to_string(),
                    size: file_size
                        .parse::<usize>()
                        .expect("File size wasn't a number"),
                };

                current_directory.files.push(file);
                println!("found file {} ({})", file_name, file_size);
            } else {
                unreachable!("Bad command");
            }
        }

        // for (name, directory) in &filesystem {
            let directory_size = calculate_directory_size(&root_directory);
            println!("{}", directory_size);

            if directory_size <= size_limit {
                total_size += directory_size;
            }
        // }

        println!("Total size: {}", total_size);
        Ok(total_size as u64)
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

fn calculate_directory_size(
    directory: &Directory,
) -> usize {
    let files_size: usize = directory.files.iter().map(|file| file.size).sum();

    let children_size: usize = directory
        .children
        .iter()
        .map(|next_dir| {
            calculate_directory_size(next_dir)
        })
        .sum();

    println!("{} + {}", files_size, children_size);

    files_size + children_size
}

test_day!(
    Day7,
    95437,
    23,
    r#"$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"#
);
