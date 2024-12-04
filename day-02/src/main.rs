use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // get all the reports
    let reports: Vec<Vec<String>> = process_reports();

    // convert the levels to integers and check if they are valid
    let safe_reports = reports.iter()
        .map(|r: &Vec<String>| parse_report(&r))
        .filter(|r: &Vec<i32>| is_valid(&r))
        .count();

    println!("Safe reports: {}", safe_reports);

    // allow buffer for the Problem Dampener
    let safe_reports = reports.iter()
        .map(|r: &Vec<String>| parse_report(&r))
        .filter(|r: &Vec<i32>| is_acceptable(&r))
        .count();

    println!("Acceptable reports: {}", safe_reports);
}

fn parse_report(report: &Vec<String>) -> Vec<i32> {
    report.iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn is_valid(vec: &Vec<i32>) -> bool {
    if vec.len() < 2 {
        return true;
    }

    let ascending: bool = vec.windows(2).all(|w: &[i32]| w[0] <= w[1]);
    let descending: bool = vec.windows(2).all(|w: &[i32]| w[0] >= w[1]);

    if !ascending && !descending {
        return false;
    }

    vec.windows(2).all(|w: &[i32]| {
        let diff = (w[1] - w[0]).abs();
        diff >= 1 && diff <= 3
    })
}

fn is_acceptable(vec: &Vec<i32>) -> bool {
    if is_valid(vec) {
        return true;
    }

    // loop through each window and slice the current index to see if that's now valid without it
    for i in 0..vec.len() {
        let new_vec = [vec[0..i].to_vec(), vec[i+1..].to_vec()].concat();
        if is_valid(&new_vec) {
            return true;
        }
    }

    false
}

fn process_reports() -> Vec<Vec<String>> {
    let file: File = File::open("puzzle.txt").expect("Failed to open file");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut reports: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        let fields: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        reports.push(fields);
    }

    reports
}