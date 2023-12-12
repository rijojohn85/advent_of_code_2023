use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = BufReader::new(File::open("input_day6").unwrap());
    let mut lines = reader.lines().map(|l| l.unwrap());
    let times_string = lines.next().unwrap();
    let vec_string = times_string.split(':').collect::<Vec<&str>>();
    let times: Vec<usize> = vec_string[1]
        .trim()
        .split("     ")
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    let distance_string = lines.next().unwrap();
    let vec_string = distance_string.split(':').collect::<Vec<&str>>();
    let distances = vec_string[1]
        .trim()
        .split("  ")
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    let mut win_options: Vec<(usize, usize)> = vec![(0, 0); times.len()];
    for (index, time) in times.iter().enumerate() {
        let distance = distances[index];
        for i in 1..=*time {
            let distance_travelled = i * (time - i);
            if distance_travelled > distance {
                let a = win_options[index].1;
                win_options[index] = (*time, a + 1);
            }
        }
    }
    let mut product = 1;
    for option in win_options {
        product *= option.1;
    }
    println!("{product}");
}
