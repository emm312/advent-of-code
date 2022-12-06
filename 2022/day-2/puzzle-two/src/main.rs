use std::fs;
fn get_game_value(a: &str, b: &str) -> usize {
    let mut ret: usize = 0;
    let a1 = match a { "A" => 1, "B" => 2, "C" => 3, _ => 1000 };
    let b1 = match b { "X" => 1, "Y" => 2, "Z" => 3, _ => 1000 };
    ret += (b1-1)*3;
    println!("{} {} {}", ret, a1, b1);
    
    if a1 == 1 {
        match b {
            "X" => ret += 3,
            "Y" => ret += 1,
            "Z" => ret += 2,
            _ => ret += 0,
        }
    } else if a1 == 2 {
        match b {
            "X" => ret += 1,
            "Y" => ret += 2,
            "Z" => ret += 3,
            _ => ret += 0,
        }
    } else if a1 == 3 {
        match b {
            "X" => ret += 2,
            "Y" => ret += 3,
            "Z" => ret += 1,
            _ => ret += 0,
        }
        println!("{}", ret);
    }
    ret
}
fn main() {
    let tmp = fs::read_to_string("input.txt").expect("puzzle input not found.");
    let content: Vec<&str> = tmp.split_whitespace().collect();
    let mut a = "";
    let mut score = 0;
    for c in content {
        if a.len() != 1 {
            a = c;
        } else {
            score += get_game_value(a, c);
            a = "";
        }
    }
    println!("{}", score);
}