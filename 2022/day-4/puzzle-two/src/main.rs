use regex::Regex;

fn main() {
    let content = include_str!("../input.txt");
    let mut out: i32 = 0;
    let regex = Regex::new(r"[0-9]+").unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for n in regex.captures_iter(content) {
        nums.push((&n[0]).parse::<i32>().unwrap());
    }

    let mut i = 0;
    while i < nums.len() {
        let from_f = nums[i].clone();
        let to_f = nums[i+1].clone();
        let from_s = nums[i+2].clone();
        let to_s = nums[i+3].clone();
        if ((from_f <= from_s) && (from_s <= to_f)) || ((from_s <= from_f) && (from_f <= to_s)) {
            out += 1;
        }
        i += 4;
    }

    eprintln!("{out}")

}
