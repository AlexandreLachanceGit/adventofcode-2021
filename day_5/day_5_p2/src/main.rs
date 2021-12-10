use std::fs::File;
use std::io::{BufRead, BufReader};

const FLOOR_SIZE: usize = 1000;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut floor = [[0; FLOOR_SIZE]; FLOOR_SIZE];
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let (mut x1, mut x2, mut y1, mut y2): (i32, i32, i32, i32);
        let mut line_iter = line.split_whitespace();

        let mut pos1_iter = line_iter.next().unwrap().split(',');
        x1 = pos1_iter.next().unwrap().parse::<i32>().unwrap();
        y1 = pos1_iter.next().unwrap().parse::<i32>().unwrap();

        line_iter.next();

        let mut pos2_iter = line_iter.next().unwrap().split(',');
        x2 = pos2_iter.next().unwrap().parse::<i32>().unwrap();
        y2 = pos2_iter.next().unwrap().parse::<i32>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                let temp = y2;
                y2 = y1;
                y1 = temp;
            }
            for i in y1..(y2 + 1) {
                floor[i as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                let temp = x2;
                x2 = x1;
                x1 = temp;
            }
            for i in x1..(x2 + 1) {
                floor[y1 as usize][i as usize] += 1;
            }
        } else {
            if x1 < x2 && y1 < y2 {
                for i in x1..(x2 + 1) {
                    floor[(i + (y1 - x1)) as usize][i as usize] += 1;
                }
            } else if x1 < x2 && y1 > y2 {
                for i in x1..(x2 + 1) {
                    floor[y1 as usize][i as usize] += 1;
                    y1 -= 1;
                }
            } else if x1 > x2 && y1 < y2 {
                for i in y1..(y2 + 1) {
                    floor[i as usize][x1 as usize] += 1;
                    x1 -= 1;
                }
            } else if x1 > x2 && y1 > y2 {
                for i in x2..(x1 + 1) {
                    floor[(i + (y2 - x2)) as usize][i as usize] += 1;
                }
            }
        }
    }

    let mut overlap_count = 0;
    for i in 0..FLOOR_SIZE {
        for j in 0..FLOOR_SIZE {
            if floor[i][j] > 1 {
                overlap_count += 1;
            }
        }
    }

    println!("{}", overlap_count);
}
