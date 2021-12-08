use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

const FIRST_PART_LEN: usize = 10;
const SECOND_PART_LEN: usize = 4;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut translation_map = HashMap::new();
    translation_map.insert("abcefg", 0);
    translation_map.insert("cf", 1);
    translation_map.insert("acdeg", 2);
    translation_map.insert("acdfg", 3);
    translation_map.insert("bcdf", 4);
    translation_map.insert("abdfg", 5);
    translation_map.insert("abdefg", 6);
    translation_map.insert("acf", 7);
    translation_map.insert("abcdefg", 8);
    translation_map.insert("abcdfg", 9);

    let mut counter = 0;
    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();
        let line_iter = line.split_whitespace();
        let mut segment: [char; 7] = Default::default();

        let mut first_part: [String; FIRST_PART_LEN] = Default::default();
        let mut second_part: [String; SECOND_PART_LEN] = Default::default();

        let mut map: HashMap<u32, Vec<String>> = HashMap::new();
        let mut i = 0;
        for word in line_iter.enumerate() {
            if i < FIRST_PART_LEN {
                first_part[i] = word.1.clone().chars().sorted().collect::<String>();
                if map.contains_key(&(first_part[i].len() as u32)) {
                    let temp = first_part[i].clone().len();
                    map.get_mut(&(temp as u32))
                        .unwrap()
                        .push(first_part[i].clone());
                } else {
                    map.insert(first_part[i].len() as u32, vec![first_part[i].clone()]);
                }
            } else if i > FIRST_PART_LEN {
                second_part[i - FIRST_PART_LEN - 1] =
                    word.1.clone().chars().sorted().collect::<String>();
            }
            i += 1;
        }

        segment[2] = map[&2][0].chars().nth(0).unwrap();
        segment[5] = map[&2][0].chars().nth(1).unwrap();

        let seven = map[&3][0].clone();
        for (_u, c) in seven.chars().enumerate() {
            if c != segment[2] && c != segment[5] {
                segment[0] = c;
            }
        }

        let four = map[&4][0].clone();
        let mut bd: Vec<char> = Vec::new();
        for (_u, c) in four.chars().enumerate() {
            if c != segment[2] && c != segment[5] {
                bd.push(c);
            }
        }

        for s in map[&6].clone() {
            if s.contains(bd[0]) && !s.contains(bd[1]) {
                segment[1] = bd[0];
                segment[3] = bd[1];
            } else if !s.contains(bd[0]) && s.contains(bd[1]) {
                segment[1] = bd[1];
                segment[3] = bd[0];
            } else if s.contains(bd[0]) && s.contains(bd[1]) {
                for (_u, c) in s.chars().enumerate() {
                    if !segment.contains(&c) && !bd.contains(&c) {
                        segment[6] = c;
                    }
                }
            }
        }
        let huit = map[&7][0].clone();
        for (_u, c) in huit.chars().enumerate() {
            if !segment.contains(&c) {
                segment[4] = c;
            }
        }

        for s in map[&6].clone() {
            if s.contains(segment[2]) && !s.contains(segment[5]) {
                let temp = segment[5];
                segment[5] = segment[2];
                segment[2] = temp;
            }
        }

        for s in map[&5].clone() {
            if s.contains(segment[0])
                && s.contains(segment[2])
                && s.contains(segment[3])
                && s.contains(segment[5])
                && !s.contains(segment[6])
            {
                let temp = segment[4];
                segment[4] = segment[6];
                segment[6] = temp;
            }
        }

        let mut values = String::new();
        for s in second_part {
            let value = translation_map[&(fix(segment, s).as_str())].to_string();
            values += &value;
        }
        counter += values.parse::<i32>().unwrap();
    }
    println!("{}", counter);
}

fn fix(segment: [char; 7], word: String) -> String {
    let mut str = String::new();

    for (_u, c) in word.chars().enumerate() {
        if c == segment[0] {
            str += "a";
        } else if c == segment[1] {
            str += "b";
        } else if c == segment[2] {
            str += "c";
        } else if c == segment[3] {
            str += "d";
        } else if c == segment[4] {
            str += "e";
        } else if c == segment[5] {
            str += "f";
        } else if c == segment[6] {
            str += "g";
        }
    }
    str.chars().sorted().collect::<String>()
}
