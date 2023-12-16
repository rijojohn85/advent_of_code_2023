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
    // let mut pc = my_map.get("AAA").unwrap();
    // let mut steps = 0;
    // let mut ctr = 0;
    //
    // loop {
    //     steps += 1;
    //     let i = instructions[ctr];
    //     let next = pc[i].clone();
    //     if &next == "ZZZ" {
    //         break;
    //     }
    //     ctr += 1;
    //     if ctr == instructions.len() {
    //         ctr = 0;
    //     }
    //     pc = my_map.get(&next).unwrap();
    // }
    // println!("{steps}");
    let mut pc = Vec::new();
    for key in my_map.keys() {
        if key.ends_with('A') {
            pc.push(key);
        }
    }
    let mut steps: usize = 0;
    let mut ctr = 0;

    // let mut cycles = vec![];
    //
    // for key in pc {
    //     let mut steps: usize = 0;
    //     let mut ctr = 0;
    //     let mut current = my_map.get(key).unwrap();
    //
    //     loop {
    //         steps += 1;
    //         let i = instructions[ctr];
    //         let next = current[i].clone();
    //         if next.ends_with('Z') {
    //             cycles.push(steps);
    //             break;
    //         }
    //         ctr += 1;
    //         if ctr == instructions.len() {
    //             ctr = 0;
    //         }
    //         current = my_map.get(&next).unwrap();
    //     }
    // }
    // println!("{:?}", cycles);
    //
    // let mut i = 0;
    // let mut j = i + 1;
    //
    // loop {
    //     let a = cycles[i];
    //     let b = cycles[j];
    //
    //     let l = lcm(a, b);
    //
    //     i += 1;
    //     j += 1;
    //     if j == cycles.len() {
    //         break;
    //     }
    // }

    loop {
        steps += 1;
        let i = instructions[ctr];
        for key in pc.iter_mut() {
            let a = my_map.get(*key).unwrap();
            *key = &a[i];
        }
        ctr += 1;
        if ctr == instructions.len() {
            ctr = 0;
        }
        let mut flag = 0;

        for key in &pc {
            if !key.ends_with('Z') {
                flag = 1;
            }
        }
        if steps % 100000000 == 0 {
            println!("{steps}");
        }

        if flag == 0 {
            break;
        }
    }
    println!("{steps}");
}
