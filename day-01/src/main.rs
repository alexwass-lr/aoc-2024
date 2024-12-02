use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // split the file into two lists
    let [mut left_list, mut right_list] = process_list();

    // sort the lists so the numbers are in order
    left_list.sort();
    right_list.sort();

    // compare the lists to calulate the distance
    let mut distance = 0;
    for i in 0..left_list.len() {
        distance += (left_list[i] - right_list[i]).abs();
    }

    println!("List distance: {}", distance);

    // compare the lists to calculate the similarity score
    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        let right_count = right_list.iter().filter(|&x| *x == left_list[i]).count();
        similarity_score += (left_list[i] * right_count as i32).abs();
    }

    println!("Similarity score: {}", similarity_score);
}

fn process_list() -> [Vec<i32>; 2] {
    let file = File::open("puzzle.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split("   ").collect();
        if fields.len() == 2 {
            left_list.push(fields[0].parse().unwrap_or(0));
            right_list.push(fields[1].parse().unwrap_or(0));
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    [left_list, right_list]
}