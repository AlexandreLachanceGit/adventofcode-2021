use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut last = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut count: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let value: i32 = line.parse::<i32>().unwrap();

        if value > last {
            count += 1;
        }
        last = value;
    }
    print!("{}\n", count);
}
