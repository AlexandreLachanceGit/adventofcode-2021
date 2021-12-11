use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut iter = BufReader::new(file).lines();

    let crabs_string = iter.next().unwrap().unwrap();
    let crabs_iter = crabs_string.split(',');
    let mut crabs_pos: Vec<i32> = Vec::new();
    for pos in crabs_iter {
        crabs_pos.push(pos.parse::<i32>().unwrap());
    }

    crabs_pos.sort();
    let median = crabs_pos[crabs_pos.len() / 2];

    let mut fuel_cost = 0;
    for pos in crabs_pos {
        fuel_cost += (median - pos).abs();
    }

    print!("{}\n", fuel_cost);
}
