use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("day8_input").unwrap());
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut instructions: Vec<usize> = vec![];
    let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
    for ch in lines.next().unwrap().chars() {
        if ch == 'R' {
            instructions.push(1);
        } else {
            instructions.push(0);
        }
    }

    for line in lines {
        if !line.is_empty() {
            let v: Vec<&str> = line.trim().split('=').collect();
            let key = v[0].trim().to_string();
            let open_bracket = v[1].find('(').unwrap();
            let close_bracket = v[1].find(')').unwrap();
            let comma = v[1].find(',').unwrap();
            let left = &v[1].trim().to_string()[open_bracket..comma - 1];
            let right = &v[1].trim().to_string()[comma + 1..close_bracket - 1];
            my_map.insert(key, vec![left.to_string(), right.to_string()]);
        }
    }
    let mut pc = my_map.get("AAA").unwrap();
    let mut steps = 0;
    let mut ctr = 0;

    loop {
        steps += 1;
        let i = instructions[ctr];
        let next = pc[i].clone();
        if &next == "ZZZ" {
            break;
        }
        ctr += 1;
        if ctr == instructions.len() {
            ctr = 0;
        }
        pc = my_map.get(&next).unwrap();
    }
    println!("{steps}");
}
