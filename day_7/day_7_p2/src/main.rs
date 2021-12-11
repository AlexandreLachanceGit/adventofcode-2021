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
    let max_value = *crabs_pos.last().unwrap();

    let mut min_fuel_cost: i128 = i128::MAX;
    for i in 0..max_value {
        let mut fuel_cost: i128 = 0;
        for pos in crabs_pos.clone() {
            let n: i128 = (i as i128 - pos as i128).abs();
            fuel_cost += (n * (n + 1)) / 2;
        }
        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
        }
    }

    print!("{}\n", min_fuel_cost);
}
