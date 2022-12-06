use std::fs;

fn get_game_value(a: &str, b: &str) -> usize {
    let mut ret: usize = 0;

    let a1 = match a { "A" => 1, "B" => 2, "C" => 3, _ => 0 };
    let b1 = match b { "X" => 1, "Y" => 2, "Z" => 3, _ => 0 };

    ret += b1;

    // draw
    if a1 == b1 { ret += 3; }

    // loss
    else if b1 == 3 && a1 == 1 || b1 == 2 && a1 == 3 || b1 == 1 && a1 == 2 {
        ret += 0;
    } else {
        // win
        ret += 6;
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
