use std::fs::File;
use std::io::{BufRead, BufReader};

const NB_DAYS: u32 = 256;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut iter = BufReader::new(file).lines();

    let fish_string = iter.next().unwrap().unwrap();
    let fish_iter = fish_string.split(',');
    let mut fish: [u128; 9] = Default::default();
    for c in fish_iter {
        fish[(c.chars().next().unwrap().to_digit(10).unwrap()) as usize] += 1;
    }

    for _i in 0..NB_DAYS {
        let fish_copy = fish.clone();
        for j in 0..8 {
            fish[j] = fish_copy[j + 1];
        }
        fish[6] += fish_copy[0];
        fish[8] = fish_copy[0];
    }

    let mut sum: u128 = 0;
    for i in 0..fish.len() {
        sum += fish[i];
    }

    print!("{}\n", sum);
}
