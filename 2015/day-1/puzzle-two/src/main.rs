use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Puzzle input not found.");


    let mut floor: i64 = 0;


    for (idx, c) in content.chars().enumerate() {
        if c == '(' { floor += 1; }
        else if c == ')' { 
            floor -= 1;
            if floor == -1 {
                eprintln!("{idx}")
            }
        }
    }
    eprintln!("{floor}");
}