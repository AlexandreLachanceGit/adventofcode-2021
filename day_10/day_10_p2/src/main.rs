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

    let mut score_map: HashMap<char, u64> = HashMap::new();
    score_map.insert('(', 1);
    score_map.insert('[', 2);
    score_map.insert('{', 3);
    score_map.insert('<', 4);

    let mut scores: Vec<u64> = Vec::new();
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let mut stack: Vec<char> = Vec::new();

        let mut is_corrupted = false;
        for c in line.clone().chars() {
            if !is_closing(c) {
                stack.push(c);
            } else {
                if pairs_map[&stack.last().unwrap()] == c {
                    stack.pop();
                } else {
                    is_corrupted = true;
                    break;
                }
            }
        }

        if !is_corrupted {
            stack.clear();
            for c in line.chars() {
                if !is_closing(c) {
                    stack.push(c);
                } else {
                    stack.pop();
                }
            }

            let mut score = 0;
            while !stack.is_empty() {
                score *= 5;
                score += score_map[&stack.pop().unwrap()];
            }
            scores.push(score);
        }
    }
    scores.sort();
    print!("{}\n", scores[scores.len() / 2]);
}

fn is_closing(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}
