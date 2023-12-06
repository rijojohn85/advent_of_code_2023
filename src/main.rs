use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut reader = BufReader::new(File::open("input_day_3").unwrap());
    let mut line = String::new();
    let mut vec_input = Vec::new();
    let mut line_len = 0;

    loop {
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        for ch in line.trim().to_string().chars() {
            vec_input.push(ch);
        }
        line_len = line.trim().len();
        line.clear();
    }
    let line_len = line_len as isize;

    let mut index_insert: Vec<usize> = Vec::new();
    let mut correct_nums = Vec::new();
    let arr: [isize; 8] = [
        (line_len - 2 * line_len - 1),
        (line_len - 2 * line_len),
        (line_len - 2 * line_len + 1),
        -1,
        1,
        (2 * line_len - line_len - 1),
        (2 * line_len - line_len),
        (2 * line_len - line_len + 1),
    ];
    for (index, char) in vec_input.iter().enumerate() {
        if char.is_numeric() {
            for i in arr {
                if (i.is_negative() && index < i.wrapping_abs() as usize)
                    || add(index, i) >= vec_input.len()
                {
                    continue;
                }
                let ch = vec_input[add(index, i)];
                if !ch.is_ascii_alphanumeric() && ch != '.' {
                    let mut j = index;
                    let mut st = String::new();
                    while vec_input[j].is_numeric() {
                        if !index_insert.contains(&j) {
                            st.push(vec_input[j]);
                            index_insert.push(j);
                        }
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    }
                    st = st.chars().rev().collect();
                    j = index + 1;
                    while vec_input[j].is_numeric() {
                        if !index_insert.contains(&j) {
                            st.push(vec_input[j]);
                            index_insert.push(j);
                        }
                        if j == vec_input.len() - 1 {
                            break;
                        }
                        j += 1;
                    }
                    if !st.is_empty() {
                        println!("char:{char}, ch:{ch}, st:{st}");
                        correct_nums.push(st.parse::<isize>().unwrap());
                    }
                }
            }
        }
    }
    // let mut sum = 0;
    // for each in correct_nums {
    //     sum += each;
    // }
    // println!("{sum}");
}

fn add(u: usize, i: isize) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
