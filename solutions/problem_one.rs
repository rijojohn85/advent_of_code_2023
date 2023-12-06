use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut input = BufReader::new(File::open("input").unwrap());
    let mut line = String::new();
    let mut sum_vec = vec![];
    loop {
        let bytes = input.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        let mut num_vec: Vec<u8> = vec![];
        for letter in line.chars() {
            if letter.is_numeric() {
                num_vec.push(letter.to_digit(10).unwrap() as u8);
            }
        }
        if num_vec.len() > 1 {
            sum_vec.push(num_vec[0] * 10 + num_vec[&num_vec.len() - 1]);
        } else {
            sum_vec.push(num_vec[0] * 10 + num_vec[0]);
        }
        line.clear();
    }
    let mut sum: usize = 0;
    for each in sum_vec {
        sum += each as usize;
    }
    println!("{}", sum);
}
