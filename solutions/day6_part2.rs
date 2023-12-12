use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = BufReader::new(File::open("input_day6").unwrap());
    let mut lines = reader.lines().map(|l| l.unwrap());
    let times_string = lines.next().unwrap();
    let vec_string = times_string.split(':').collect::<Vec<&str>>();
    let times = vec_string[1].trim().split_ascii_whitespace();
    let mut time = String::new();
    for a in times {
        time.push_str(a);
    }
    let time: usize = time.parse().unwrap();
    let distance_string = lines.next().unwrap();
    let vec_string = distance_string.split(':').collect::<Vec<&str>>();
    let distances = vec_string[1].trim().split_ascii_whitespace();
    let mut distance = String::new();
    for a in distances {
        distance.push_str(a);
    }
    let distance: usize = distance.parse().unwrap();
    println!("time: {time}, disance: {distance}");
    let mut win_options = 0;
    for i in 1..=time {
        let distance_travelled = i * (time - i);
        if distance_travelled > distance {
            win_options += 1;
        }
    }
    println!("{win_options}");
    // let mut win_options: Vec<(usize, usize)> = vec![(0, 0); times.len()];
    // for (index, time) in times.iter().enumerate() {
    //     let distance = distances[index];
    //     for i in 1..=*time {
    //         let distance_travelled = i * (time - i);
    //         if distance_travelled > distance {
    //             let a = win_options[index].1;
    //             win_options[index] = (*time, a + 1);
    //         }
    //     }
    // }
    // let mut product = 1;
    // for option in win_options {
    //     product *= option.1;
    // }
    // println!("{product}");
}
