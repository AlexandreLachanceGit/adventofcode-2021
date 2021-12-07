use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut iter = BufReader::new(file).lines();

    let mut count: i32 = 0;
    let mut window_a: [i32; 3] = [0, 0, 0];
    for i in 0..3 {
        window_a[i] = iter.next().unwrap().unwrap().parse::<i32>().unwrap();
    }
    let mut window_b: [i32; 3] = [0, 0, 0];

    for (_index, line) in iter.enumerate() {
        window_b[0] = window_a[1];
        window_b[1] = window_a[2];
        window_b[2] = line.unwrap().parse::<i32>().unwrap();

        if sum(window_b) > sum(window_a) {
            count += 1;
        }
        window_a = window_b;
    }
    print!("{}\n", count);
}

fn sum(window: [i32; 3]) -> i32 {
    window[0] + window[1] + window[2]
}
