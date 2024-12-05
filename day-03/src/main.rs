use std::fs;
use regex::Regex;

fn main() {
    let corrupted_memory: String = fs::read_to_string("puzzle.txt").expect("Failed to read file");

    // find all the mul(x,y) values
    let pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&corrupted_memory).map(|matched| matched.as_str()).collect();

    let result = process_calculations(matches);

    println!("Result: {}", result);

    // split the string into instruction parts because a regex look-around is not supported in Rust
    let enabled_string: String = removed_disabled(&corrupted_memory).join("");
    let enabled_matches: Vec<&str> = pattern.find_iter(&enabled_string).map(|matched| matched.as_str()).collect();

    let enabled_result = process_calculations(enabled_matches);
    println!("Enabled calculations: {}", enabled_result);
}

fn removed_disabled(input: &str) -> Vec<&str> {
    let mut results = Vec::new();
    let mut start = 0;

    // start with do()
    let mut current_prefix = "do()";
    let markers = ["do()", "don't()"];

    // loop through the input finding each marker and it's preceding text
    while start < input.len() {
        // get the next position of each marker and get the first one
        let next_pos = markers
            .iter()
            .filter_map(|&marker| input[start..].find(marker).map(|pos| (marker, pos + start)))
            .min_by_key(|&(_, pos)| pos);

        if let Some((marker, pos)) = next_pos {
            if current_prefix == "do()" {
                // we only need valid segments
                results.push(&input[start..pos]);
            }

            current_prefix = marker;
            start = pos + marker.len();
        } else {
            // capture the remaining string
            if current_prefix == "do()" {
                results.push(&input[start..]);
            }
            break;
        }
    }

    results
}

fn process_calculations(matches: Vec<&str>) -> i32 {
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

    result
}