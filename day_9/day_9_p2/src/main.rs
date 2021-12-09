use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pos {
    number: u32,
    is_part_of_basin: bool,
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut heightmap: Vec<Vec<Pos>> = Vec::new();
    for (_index, line) in iter.enumerate() {
        let mut heightmap_line: Vec<Pos> = Vec::new();
        let line = line.unwrap();

        for n in line.chars() {
            let new_pos: Pos = Pos {
                number: n.to_digit(10).unwrap(),
                is_part_of_basin: false,
            };
            heightmap_line.push(new_pos);
        }
        heightmap.push(heightmap_line);
    }

    let mut basin_sizes: Vec<u32> = Vec::new();
    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            let mut is_low_point = true;

            // Up
            if y + 1 < heightmap.len() && heightmap[y + 1][x].number <= heightmap[y][x].number {
                is_low_point = false;
            }
            // Down
            if y != 0 && heightmap[y - 1][x].number <= heightmap[y][x].number {
                is_low_point = false;
            }
            // Right
            if x + 1 < heightmap[0].len() && heightmap[y][x + 1].number <= heightmap[y][x].number {
                is_low_point = false;
            }
            // Left
            if x != 0 && heightmap[y][x - 1].number <= heightmap[y][x].number {
                is_low_point = false;
            }

            if is_low_point {
                heightmap[y][x].is_part_of_basin = true;
                let size = get_basin_size(&mut heightmap, x, y, 1);
                basin_sizes.push(size);
                print!("{} ", size);
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    println!(
        "\n3 largest: {} {} {}\nAnswer : {}",
        basin_sizes[0],
        basin_sizes[1],
        basin_sizes[2],
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    );
}

fn get_basin_size(heightmap: &mut Vec<Vec<Pos>>, x: usize, y: usize, current_size: u32) -> u32 {
    let mut new_size = current_size;
    heightmap[y][x].is_part_of_basin = true;

    // Up
    if y + 1 < heightmap.len()
        && !heightmap[y + 1][x].is_part_of_basin
        && heightmap[y + 1][x].number != 9
    {
        new_size += get_basin_size(heightmap, x, y + 1, current_size);
    }
    // Down
    if y != 0 && !heightmap[y - 1][x].is_part_of_basin && heightmap[y - 1][x].number != 9 {
        new_size += get_basin_size(heightmap, x, y - 1, current_size);
    }
    // Right
    if x + 1 < heightmap[0].len()
        && !heightmap[y][x + 1].is_part_of_basin
        && heightmap[y][x + 1].number != 9
    {
        new_size += get_basin_size(heightmap, x + 1, y, current_size);
    }
    // Left
    if x != 0 && !heightmap[y][x - 1].is_part_of_basin && heightmap[y][x - 1].number != 9 {
        new_size += get_basin_size(heightmap, x - 1, y, current_size);
    }
    new_size
}
