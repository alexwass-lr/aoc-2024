use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let corrupted_memory: String = fs::read_to_string("puzzle.txt").expect("Failed to read file");

    // find all the mul(x,y) values
    let pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&corrupted_memory).map(|matched| matched.as_str()).collect();

    let mut result: i32 = 0;

    for matched in matches {
        // extract the x and y values
        let values: Vec<i32> = matched
            .split(',')
            .filter_map(|string| {
                let cleaned: String = string.chars().filter(|c| c.is_numeric()).collect();
                cleaned.parse::<i32>().ok()
            })
            .collect();

        result += values[0] * values[1];
    }

    println!("Result: {}", result);
}
