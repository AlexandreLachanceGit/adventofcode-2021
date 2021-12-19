use std::fs::File;
use std::io::{BufRead, BufReader};

const NB_STEPS: u32 = 100;

struct Octopus {
    energy_level: u8,
    flashed: bool,
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut grid: Vec<Vec<Octopus>> = Vec::new();
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let mut line_vec: Vec<Octopus> = Vec::new();
        for c in line.chars() {
            line_vec.push(Octopus {
                energy_level: c.to_digit(10).unwrap() as u8,
                flashed: false,
            });
        }
        grid.push(line_vec);
    }

    let mut flashes = 0;
    for _k in 0..NB_STEPS {
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                grid[i][j].energy_level += 1;
                grid[i][j].flashed = false;
            }
        }
        while contains_bigger_than_nine(&grid) {
            for i in 0..grid.len() {
                for j in 0..grid.len() {
                    if grid[i][j].energy_level > 9 && !grid[i][j].flashed {
                        grid[i][j].flashed = true;
                        flashes += 1;
                        grid[i][j].energy_level = 0;
                        if i + 1 < grid.len() && !grid[i + 1][j].flashed {
                            grid[i + 1][j].energy_level += 1;
                        }
                        if j + 1 < grid.len() && !grid[i][j + 1].flashed {
                            grid[i][j + 1].energy_level += 1;
                        }
                        if i != 0 && !grid[i - 1][j].flashed {
                            grid[i - 1][j].energy_level += 1;
                        }
                        if j != 0 && !grid[i][j - 1].flashed {
                            grid[i][j - 1].energy_level += 1;
                        }
                        if i + 1 < grid.len() && j + 1 < grid.len() && !grid[i + 1][j + 1].flashed {
                            grid[i + 1][j + 1].energy_level += 1;
                        }
                        if i != 0 && j + 1 < grid.len() && !grid[i - 1][j + 1].flashed {
                            grid[i - 1][j + 1].energy_level += 1;
                        }
                        if i != 0 && j != 0 && !grid[i - 1][j - 1].flashed {
                            grid[i - 1][j - 1].energy_level += 1;
                        }
                        if j != 0 && i + 1 < grid.len() && !grid[i + 1][j - 1].flashed {
                            grid[i + 1][j - 1].energy_level += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", flashes);
}

fn contains_bigger_than_nine(grid: &Vec<Vec<Octopus>>) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j].energy_level > 9 && !grid[i][j].flashed {
                return true;
            }
        }
    }
    false
}
