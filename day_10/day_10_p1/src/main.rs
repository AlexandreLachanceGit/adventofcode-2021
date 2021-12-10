use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut pairs_map: HashMap<char, char> = HashMap::new();
    pairs_map.insert('(', ')');
    pairs_map.insert('[', ']');
    pairs_map.insert('{', '}');
    pairs_map.insert('<', '>');

    let mut score_map: HashMap<char, u32> = HashMap::new();
    score_map.insert(')', 3);
    score_map.insert(']', 57);
    score_map.insert('}', 1197);
    score_map.insert('>', 25137);

    let mut score = 0;
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if !is_closing(c) {
                stack.push(c);
            } else {
                if pairs_map[&stack.last().unwrap()] == c {
                    stack.pop();
                } else {
                    score += score_map[&c];
                    break;
                }
            }
        }
    }
    print!("{}\n", score);
}

fn is_closing(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}
