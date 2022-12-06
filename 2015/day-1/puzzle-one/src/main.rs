use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Puzzle input not found.");


    let mut floor: i64 = 0;


    for c in content.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => floor = floor,
        }
    }
    eprintln!("{floor}");
}