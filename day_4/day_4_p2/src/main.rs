use std::fs::File;
use std::io::{BufRead, BufReader};

const NB_BOARDS: i32 = 100;

struct Board {
    tiles: [[Tile; 5]; 5],
}

impl Board {
    fn update(&mut self, number: i32) -> i32 {
        for i in 0..5 {
            for j in 0..5 {
                if self.tiles[i][j].number == number {
                    self.tiles[i][j].is_marked = true;
                }
            }
        }

        if self.check_if_bingo() {
            return self.sum();
        }
        0
    }

    fn check_if_bingo(&self) -> bool {
        let mut vertical = true;
        let mut horizontal = true;
        for i in 0..5 {
            for j in 0..5 {
                if !self.tiles[i][j].is_marked {
                    horizontal = false;
                }
                if !self.tiles[j][i].is_marked {
                    vertical = false;
                }
            }
            if vertical || horizontal {
                return true;
            }
            vertical = true;
            horizontal = true;
        }
        false
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.tiles[i][j].is_marked {
                    sum += self.tiles[i][j].number;
                }
            }
        }
        sum
    }
}

#[derive(Default)]
struct Tile {
    number: i32,
    is_marked: bool,
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut iter = BufReader::new(file).lines();

    let steps_line = iter.next().unwrap().unwrap();
    let steps_iter = steps_line.split(",");

    let mut steps = Vec::new();
    for s in steps_iter {
        steps.push(s.parse::<i32>().unwrap());
    }

    let mut boards: Vec<Board> = Vec::new();
    for _i in 0..NB_BOARDS {
        let mut tiles: [[Tile; 5]; 5] = Default::default();
        iter.next();
        for i in 0..5 {
            let mut j = 0;
            for n in iter.next().unwrap().unwrap().split_whitespace() {
                tiles[i][j] = Tile {
                    number: n.parse::<i32>().unwrap(),
                    is_marked: false,
                };
                j += 1;
            }
        }
        let new_board = Board { tiles: tiles };
        boards.push(new_board);
    }

    let mut wins: Vec<usize> = Vec::new();
    for s in steps {
        for b in 0..boards.len() {
            let val = boards[b].update(s);
            if val != 0 {
                if !wins.contains(&b) {
                    wins.push(b);
                }
            }

            if wins.len() == 100 {
                println!("board:{} sum:{} number:{} score:{}", b, val, s, val * s);
                return;
            }
        }
    }
}
