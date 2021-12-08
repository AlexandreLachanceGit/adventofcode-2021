use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal_pos: i32 = 0;

    for (_index, line) in iter.enumerate() {
        let command_line = line.unwrap();
        let mut command_iter = command_line.split_whitespace();
        let command = command_iter.next().unwrap();
        let value = command_iter.next().unwrap().parse::<i32>().unwrap();

        match command {
            "forward" => forward(&mut horizontal_pos, &mut depth, aim, value),
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!(),
        }
    }
    print!("{}\n", depth * horizontal_pos);
}

fn forward(horizontal_pos: &mut i32, depth: &mut i32, aim: i32, value: i32) {
    *horizontal_pos += value;
    *depth += value * aim;
}
