use std::fs::File;
use std::io::{BufRead, BufReader};

const NB_DAYS: u32 = 80;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut iter = BufReader::new(file).lines();

    let fish_string = iter.next().unwrap().unwrap();
    let fish_iter = fish_string.split(',');
    let mut fish: Vec<u32> = Vec::new();
    for c in fish_iter {
        fish.push(c.chars().next().unwrap().to_digit(10).unwrap());
    }

    for _i in 0..NB_DAYS {
        for j in 0..fish.len() {
            if fish[j] == 0 {
                fish[j] = 6;
                fish.push(8);
            } else {
                fish[j] -= 1;
            }
        }
    }

    print!("{}\n", fish.len());
}
