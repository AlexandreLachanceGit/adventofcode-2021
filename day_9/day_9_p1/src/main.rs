use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut heightmap: Vec<Vec<u32>> = Vec::new();
    for (_index, line) in iter.enumerate() {
        let mut heightmap_line: Vec<u32> = Vec::new();
        let line = line.unwrap();

        for n in line.chars() {
            heightmap_line.push(n.to_digit(10).unwrap());
        }
        heightmap.push(heightmap_line);
    }

    let mut sum = 0;
    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            let mut is_low_point = true;

            // Up
            if y + 1 < heightmap.len() && heightmap[y + 1][x] <= heightmap[y][x] {
                is_low_point = false;
            }
            // Down
            if y != 0 && heightmap[y - 1][x] <= heightmap[y][x] {
                is_low_point = false;
            }
            // Right
            if x + 1 < heightmap[0].len() && heightmap[y][x + 1] <= heightmap[y][x] {
                is_low_point = false;
            }
            // Left
            if x != 0 && heightmap[y][x - 1] <= heightmap[y][x] {
                is_low_point = false;
            }

            if is_low_point {
                print!("{} ", heightmap[y][x]);
                sum += heightmap[y][x] + 1;
            }
        }
    }

    println!("\nSum of risk levels : {}", sum);
}
