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
    let mut content = include_str!("../input.txt").lines().peekable();
    let mut result = 0;
    while content.peek().is_some() {
        let bits1 = value(content.next().unwrap());
        let bits2 = value(content.next().unwrap());
        let bits3 = value(content.next().unwrap());
    
        let common = bits1 & bits2 & bits3;
        result += u64::BITS - common.leading_zeros();
    }

    eprintln!("{result}");    
}
