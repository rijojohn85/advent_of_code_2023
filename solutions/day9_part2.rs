use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("day9_input").unwrap());
    let lines = reader.lines().map(|l| l.unwrap());
    let mut input: Vec<Vec<isize>> = vec![];

    for line in lines {
        let mut l = vec![];
        if !line.is_empty() {
            let v: Vec<&str> = line.trim().split(' ').collect();
            for a in v {
                l.push(a.parse::<isize>().unwrap());
            }
        }
        input.push(l);
    }
    let sum: isize = input.iter().map(find_prev).sum();
    // for each in input {
    //     let s = find_prev(&each);
    //     println!("{s}");
    // }
    println!("{sum}");
    // println!("{:?}", find_prev(&input[2]));
}

fn find_prev(input: &Vec<isize>) -> isize {
    let mut output: Vec<Vec<isize>> = vec![input.clone()];
    let mut i = 0;
    let mut j = i + 1;
    let mut temp = vec![];
    let mut sum = 0;
    loop {
        let l = output.len() - 1;
        let diff = output[l][j] - output[l][i];
        temp.push(diff);
        i += 1;
        j += 1;

        if j == output[l].len() {
            output.push(temp.clone());
            if check_equal_vec(&temp) {
                output.reverse();
                for each in &output {
                    let e = each.first().unwrap();
                    sum = e - sum;
                }

                break;
            }
            temp.clear();
            i = 0;
            j = i + 1;
        }
    }

    sum
}

fn check_equal_vec(input: &Vec<isize>) -> bool {
    let check = input[0];
    for each in input {
        if each != &check {
            return false;
        }
    }

    true
}
