use std::fs;

fn value(line: &str) -> u64 {
    line.chars()
        .map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => panic!("{c}"),
        })
        .fold(0, |bits, bit| bits | 1 << bit)
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("puzzle input not found");
    let out = content.clone()
    .lines()
    .map(|line| {
        let (part1, part2) = line.split_at(line.len() / 2);

        let common = value(part1) & value(part2);
        u64::BITS - common.leading_zeros()
    })
    .sum::<u32>();


    eprintln!("{out}");
    

    
}
