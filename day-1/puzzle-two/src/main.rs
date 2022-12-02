use std::{fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Puzzle input does not exist.");
    let mut calories_arr: Vec<u64> = Vec::new();
    let mut curr_calories: u64 = 0;
    for line in content.lines() {
        let curr = line.parse::<u64>();
        if curr.is_ok() {
            curr_calories += curr.unwrap();
        } else {
            calories_arr.push(curr_calories);
            curr_calories = 0;
        }
        
    }
    calories_arr.sort();
    calories_arr.reverse();
    println!("{}", calories_arr[0]+calories_arr[1]+calories_arr[2]);
}
