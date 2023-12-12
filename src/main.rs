use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = BufReader::new(File::open("input_day7_test").unwrap());
    let mut games: Vec<(String, usize)> = vec![];
    let lines = reader.lines().map(|l| l.unwrap());
    for line in lines {
        let v = line.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        let hand = v[0].trim().to_string();
        let bid = v[1].trim().parse::<usize>().unwrap();
        games.push((hand, bid));
    }
    println!("{:?}", games);
}
