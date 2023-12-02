use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::Day;

const MODULE_TEMPLATE: &str = r#"advent_of_code::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u32> {
    advent_of_code::solutions::dayDAYPART::part_one(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    advent_of_code::solutions::dayDAYPART::part_two(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    // #[test]
    // #[cfg_attr(feature = "ci", ignore)]
    // fn test_part_one_input() {
    //     let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // #[cfg_attr(feature = "ci", ignore)]
    // fn test_part_two_input() {
    //     let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, None);
    // }
}
"#;

const SOLUTIONS_TEMPLATE: &str = r#"use crate::solutions::common::split_into_lines;

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn create_parent_dir(path: &str) -> Result<(), std::io::Error> {
    let parent_dir = std::path::Path::new(path).parent().unwrap();
    std::fs::create_dir_all(parent_dir)
}

pub fn handle(day: Day) {
    let input_path = format!("data/inputs/{day}.txt");
    let example_path = format!("data/examples/{day}.txt");
    let module_path = format!("src/bin/{day}.rs");
    let solutions_path = format!("src/solutions/day{day}/mod.rs");

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .replace("DAYPART", format!("{day}").as_str())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match create_parent_dir(&solutions_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to create solutions directory: {e}");
            process::exit(1);
        }
    }

    let mut solution_file = match safe_create_file(&solutions_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create solutions file: {e}");
            process::exit(1);
        }
    };

    match solution_file.write_all(SOLUTIONS_TEMPLATE.as_bytes()) {
        Ok(()) => {
            println!("Created solutions file \"{}\"", &solutions_path);
        }
        Err(e) => {
            eprintln!("Failed to write solutions contents: {e}");
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {}` to run your solution.", day);
}
