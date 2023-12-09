use ndarray::{Array2, Axis};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut reader = BufReader::new(File::open("input_day_3").unwrap());
    let mut line = String::new();
    let mut line_len = 0;
    let mut num_lines = 0;

    let mut a1 = vec![];
    loop {
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        for char in line.trim().to_string().chars() {
            a1.push(char);
        }
        line_len = line.trim().len();
        line.clear();
        num_lines += 1;
    }

    let a = Array2::from_shape_vec((line_len, num_lines), a1).unwrap();

    let mut index_insert: Vec<(usize, usize)> = Vec::new();
    let mut correct_nums: Vec<usize> = Vec::new();
    for (i, a1) in a.axis_iter(Axis(0)).enumerate() {
        for (j, a2) in a1.iter().enumerate() {
            if !a2.is_numeric() && a2 != &'.' {
                let coord = get_coord(i, j);
                for c in coord {
                    if let Some(ch) = a.get(c) {
                        if ch.is_numeric() {
                            let mut st = String::new();
                            let x = c.0;
                            let mut y = c.1;

                            while a.get((x, y)).unwrap().is_numeric() {
                                if !index_insert.contains(&(x, y)) {
                                    st.push(*a.get((x, y)).unwrap());
                                    index_insert.push((x, y));
                                }
                                if y == 0 {
                                    break;
                                }
                                y -= 1;
                            }
                            st = st.chars().rev().collect();
                            y = c.1 + 1;
                            if y < line_len {
                                while a.get((x, y)).unwrap().is_numeric() {
                                    if !index_insert.contains(&(x, y)) {
                                        st.push(*a.get((x, y)).unwrap());
                                        index_insert.push((x, y));
                                    }
                                    if y == line_len - 1 {
                                        break;
                                    }
                                    y += 1;
                                }
                            }
                            if !st.is_empty() {
                                correct_nums.push(st.parse().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for each in correct_nums {
        sum += each;
    }
    println!("{sum}");
}

fn get_coord(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if i > 0 {
        result.push((i - 1, j));
        result.push((i - 1, j + 1));

        if j > 0 {
            result.push((i - 1, j - 1));
        }
    }
    if j > 0 {
        result.push((i, j - 1));
        result.push((i + 1, j - 1));
    }

    result.push((i, j + 1));
    result.push((i + 1, j));
    result.push((i + 1, j + 1));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_coord() {
        assert_eq!(get_coord(0, 0), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(
            get_coord(1, 0),
            vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]
        );
        assert_eq!(
            get_coord(0, 1),
            vec![(0, 0), (1, 0), (0, 2), (1, 1), (1, 2)]
        );
        assert_eq!(
            get_coord(1, 1),
            vec![
                (0, 1),
                (0, 2),
                (0, 0),
                (1, 0),
                (2, 0),
                (1, 2),
                (2, 1),
                (2, 2)
            ]
        );
    }
}
