use regex::Regex;

fn main() {
    let content = include_str!("../input.txt");
    let mut out = 0;
    let regex = Regex::new(r"[0-9]+").unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for n in regex.captures_iter(content) {
        nums.push((&n[0]).parse::<i32>().unwrap());
    }

    let mut i = 0;
    while i < nums.len() {
        if nums[i] <= nums[i+2] && nums[i+1] >= nums[i+3] || nums[i] >= nums[i+2] && nums[i+1] <= nums[i+3] {
            out += 1;
        }
        i += 4;
    }

    eprintln!("{out}")

}
