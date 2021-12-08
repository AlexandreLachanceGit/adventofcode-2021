use std::fs::File;
use std::io::{BufRead, BufReader};

const FIRST_PART_LEN: usize = 10;
const SECOND_PART_LEN: usize = 4;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut counter = 0;
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let line_iter = line.split_whitespace();
        let mut first_part: [String; FIRST_PART_LEN] = Default::default();
        let mut second_part: [String; SECOND_PART_LEN] = Default::default();

        let mut i = 0;
        for word in line_iter.enumerate() {
            if i < FIRST_PART_LEN {
                first_part[i] = word.1.into();
            } else if i > FIRST_PART_LEN {
                second_part[i - FIRST_PART_LEN - 1] = word.1.into();
                let len = second_part[i - FIRST_PART_LEN - 1].len();
                if len == 2 || len == 3 || len == 4 || len == 7 {
                    counter += 1;
                }
            }
            i += 1;
        }
    }
    println!("{}", counter);
}
