use std::fs::File;
use std::io::{self, BufRead};

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // Top left
    (-1, 0),  // Top
    (-1, 1),  // Top right
    (0, 1),   // Right
    (1, 1),   // Bottom right
    (1, 0),   // Bottom
    (1, -1),  // Bottom left
    (0, -1),  // Left
];

fn main() {
    let puzzle: Vec<Vec<String>> = load_puzzle();
    let mut found: i32 = 0;

    for (i, row) in puzzle.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            // only start looking when we find an X
            if letter == "X" {
                found += find_word(&puzzle, i, j, letter.to_string());
            }
        }
    }

    println!("Found {} words", found);
}

fn find_word(puzzle: &Vec<Vec<String>>, i: usize, j: usize, letter: String) -> i32 {
    let mut found: i32 = 0;

    // get all the possible sibling positions around X to check
    for (dir, (di, dj)) in DIRECTIONS.iter().enumerate() {
        let ni = (i as isize + di) as usize;
        let nj = (j as isize + dj) as usize;

        if ni < puzzle.len() && nj < puzzle[ni].len() {
            if check_next_position(&puzzle, ni, nj, letter.clone(), dir as usize) {
                found += 1;
            }
        }
    }

    found
}

fn check_next_position(puzzle: &Vec<Vec<String>>, i: usize, j: usize, letter: String, direction: usize) -> bool {
    let next_letter = match letter.as_str() {
        "X" => "M",
        "M" => "A",
        "A" => "S",
        "S" => return true,
        _ => return false,
    };

    // get the next position difference
    let (di, dj) = DIRECTIONS[direction];

    // if we find the next letter, recursively check in the current direction for the remaining letters
    if i < puzzle.len() && j < puzzle[i].len() && puzzle[i][j] == next_letter {
        let ni = (i as isize + di) as usize;
        let nj = (j as isize + dj) as usize;
        if check_next_position(puzzle, ni, nj, next_letter.to_string(), direction) {
            return true;
        }
    }

    false
}

fn load_puzzle() -> Vec<Vec<String>> {
    let file: File = File::open("puzzle.txt").expect("Failed to open file");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut puzzle: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        let fields: Vec<String> = line.split("").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        puzzle.push(fields);
    }

    puzzle
}