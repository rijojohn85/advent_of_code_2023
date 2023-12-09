use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut reader = BufReader::new(File::open("input_day4_part1").unwrap());
    let mut line = String::new();
    let mut sum = 0;

    loop {
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        let v: Vec<&str> = line.split(':').collect();
        let v1: Vec<&str> = v[1].split('|').collect();
        let results = str_to_vec(v1[0].trim());
        let our_nums = str_to_vec(v1[1].trim());
        let mut n = 0;
        for each in our_nums {
            if results.contains(&each) {
                n += 1;
            }
        }
        if n != 0 {
            let base: i32 = 2;
            sum += base.pow(n - 1);
        }

        line.clear();
    }
    println!("{sum}");
}
fn str_to_vec(input: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let v: Vec<&str> = input.split(' ').collect();

    for each in v {
        if let Ok(n) = each.parse::<usize>() {
            result.push(n);
        }
    }
    result
}
