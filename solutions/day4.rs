use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut reader = BufReader::new(File::open("input_day5_test").unwrap());
    let mut line = String::new();

    loop {
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        print!("{line}");

        line.clear();
    }
}
