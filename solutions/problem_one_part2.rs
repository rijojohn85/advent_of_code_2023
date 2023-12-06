use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut input = BufReader::new(File::open("input").unwrap());
    let mut line = String::new();
    let mut sum = 0;
    let mut vec_sum = vec![];
    loop {
        let mut sum_vec = vec![];
        let bytes = input.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        let num_str = str_to_num(line.clone());
        for ch in num_str.chars() {
            if ch.is_numeric() {
                sum_vec.push(ch.to_digit(10).unwrap());
            }
        }
        vec_sum.push(sum_vec[0] * 10 + sum_vec[&sum_vec.len() - 1]);
        line.clear();
    }

    for each in vec_sum {
        sum += each;
    }
    println!("{sum}");
}

fn str_to_num(input: String) -> String {
    let mut num_map = HashMap::new();
    num_map.insert("one", "o1e");
    num_map.insert("two", "t2o");
    num_map.insert("three", "t3ree");
    num_map.insert("four", "f4ur");
    num_map.insert("five", "f5ve");
    num_map.insert("six", "s6x");
    num_map.insert("seven", "s7ven");
    num_map.insert("eight", "e8ght");
    num_map.insert("nine", "n9ne");
    let mut an = input.clone();
    // for i in 0..an.len() {
    //     for j in i..an.len() {
    //         let s = &an[i..j];
    //         if let Some(num) = num_map.get(s) {
    //             an = an.replace(s, num);
    //         }
    //     }
    // }
    for (key, value) in num_map {
        an = an.replace(key, value);
    }
    an
}
