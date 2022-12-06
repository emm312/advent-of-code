fn main() {
    let content: Vec<char> = include_str!("../input.txt").chars().collect();

    for i in 0..content.len()-14 {
        let mut chars: Vec<char> = vec![content[i],content[i+1],content[i+2],content[i+3],content[i+4],content[i+5],content[i+6],content[i+7],content[i+8],content[i+9],content[i+10],content[i+11],content[i+12],content[i+13]];

        chars.sort();
        chars.dedup();
        if chars.len() == 14 {
            println!("{}", i+14);
            break;
        }
    }
}