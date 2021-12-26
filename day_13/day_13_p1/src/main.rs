use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

struct Fold {
    is_x: bool,
    pos: usize,
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut max_x = 0;
    let mut max_y = 0;
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        if !line.is_empty() && !line.contains("fold") {
            let mut line_iter = line.split(",");
            let first = line_iter.next().unwrap().parse::<usize>().unwrap();
            let second = line_iter.next().unwrap().parse::<usize>().unwrap();
            if first > max_x {
                max_x = first;
            }
            if second > max_y {
                max_y = second;
            }
        }
    }
    max_x += 1;
    max_y += 1;

    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();
    let mut sheet = vec![vec![false; max_x]; max_y];
    let mut folds: Vec<Fold> = Vec::new();
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        if !line.is_empty() {
            if line.contains("x") {
                let mut line_iter = line.split("=");
                line_iter.next();
                folds.push(Fold {
                    is_x: true,
                    pos: line_iter.next().unwrap().parse::<usize>().unwrap(),
                });
            } else if line.contains("y") {
                let mut line_iter = line.split("=");
                line_iter.next();
                folds.push(Fold {
                    is_x: false,
                    pos: line_iter.next().unwrap().parse::<usize>().unwrap(),
                });
            } else {
                let mut line_iter = line.split(",");
                let first = line_iter.next().unwrap().parse::<usize>().unwrap();
                let second = line_iter.next().unwrap().parse::<usize>().unwrap();
                sheet[second][first] = true;
            }
        }
    }

    let fold = &folds[0];
    let mut new_sheet: Vec<Vec<bool>>;
    if fold.is_x {
        new_sheet = vec![vec![false; max_x / 2]; max_y];
        for x in 0..fold.pos as usize {
            for y in 0..max_y {
                new_sheet[y][x] = sheet[y][x] || sheet[y][fold.pos * 2 - x];
            }
        }
    } else {
        new_sheet = vec![vec![false; max_x]; max_y / 2];
        for y in 0..fold.pos as usize {
            for x in 0..max_x {
                new_sheet[y][x] = sheet[y][x] || sheet[fold.pos * 2 - y][x];
            }
        }
    }

    println!("{}", count_sheet(new_sheet));
}

fn count_sheet(sheet: Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for y in 0..sheet.len() {
        for x in 0..sheet[0].len() {
            if sheet[y][x] {
                count += 1;
            }
        }
    }
    count
}
