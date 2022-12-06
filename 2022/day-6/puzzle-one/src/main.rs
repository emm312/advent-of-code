fn main() {
    let content: Vec<char> = include_str!("../input.txt").chars().collect();

    for i in 0..content.len()-4 {
        let mut chars: Vec<char> = vec![content[i], content[i+1], content[i+2], content[i+3]];
        chars.sort();
        chars.dedup();
        if chars.len() == 4 {
            println!("{}", i+4);
            break;
        }
    }
}
