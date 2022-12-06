use regex::Regex;

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for c in stacks {
        for x in c {
            print!("{}", x);
        }
        println!();
    }
}

fn main() {
    let content = include_str!("../input.txt");

    let content_lines: Vec<&str> = content.lines().collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for l in content_lines {
        if l.len() != 0 {
            stacks.push(l.chars().collect()); // 2d array of characters go brr
        } else {
            break;
        }
    }

    let mut x = 1;

    let mut parsed_stacks: Vec<Vec<char>> = Vec::new();
    let mut curr_stack = 0;
    while x < stacks[0].len() {
        parsed_stacks.push(Vec::new());
        let mut y = 0;
        while y < stacks.len() {
            if stacks[y][x].is_alphabetic() {
                parsed_stacks[curr_stack].push(stacks[y][x]);
            }
            y += 1;
        }
        parsed_stacks[curr_stack].reverse();
        curr_stack += 1;
        x += 4;
    }

    let lines: Vec<&str> = content.lines().collect();
    let insts: &str = &lines.split_at(stacks.len()).1.join("\n");
    let inst_regex = Regex::new(r"[0-9]+").unwrap();

    let captures: Vec<usize> = inst_regex.find_iter(insts).map(|c| c.as_str().parse::<usize>().unwrap() ).collect();

    let mut i = 0;
    while i < captures.len() {
        let amount = captures[i].clone();
        let from = captures[i+1].clone();
        let to = captures[i+2].clone();

        let mut end = parsed_stacks[from-1].split_at(parsed_stacks[from-1].len()-amount).1.to_vec();

        for _ in 0..amount {
            parsed_stacks[from-1].pop();
        }


        parsed_stacks[to-1].append(&mut end);
        i += 3;
    }

    print_stacks(&parsed_stacks);
    for stack in parsed_stacks {
        if stack.len() != 0 {
            print!("{}", stack[stack.len()-1]);
        } else {
            println!("");
        }
    }
    println!();
}
