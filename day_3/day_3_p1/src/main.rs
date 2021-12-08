use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let nb_lines = 1000;
    let mut one_count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let chars = line.chars();
        let mut i = 0;
        for c in chars {
            if c == '1' {
                one_count[i] += 1;
            }
            i += 1;
        }
    }

    let mut gamma_rate_str: String = String::new();
    let mut epsilon_rate_str: String = String::new();
    for i in one_count {
        if i > nb_lines / 2 {
            gamma_rate_str += "1";
            epsilon_rate_str += "0";
        } else {
            gamma_rate_str += "0";
            epsilon_rate_str += "1";
        }
    }

    print!(
        "{}\n",
        isize::from_str_radix(gamma_rate_str.as_str(), 2).unwrap()
            * isize::from_str_radix(epsilon_rate_str.as_str(), 2).unwrap()
    );
}
