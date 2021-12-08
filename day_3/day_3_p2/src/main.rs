use std::fs::File;
use std::io::{BufRead, BufReader};

const NB_CHAR: usize = 12;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let iter = BufReader::new(file).lines();

    let mut one_count: [i32; NB_CHAR] = Default::default();
    let mut oxygen_generator_rating: String = String::new();
    let mut co2_scrubber_rating: String = String::new();
    let mut numbers: Vec<Vec<u32>> = Vec::new();

    for (_index, line) in iter.enumerate() {
        let line = line.unwrap();

        let mut new_vec: Vec<u32> = Vec::new();
        for c in line.chars().enumerate() {
            let c = c.1.to_digit(10).unwrap();
            new_vec.push(c);
        }
        numbers.push(new_vec);

        let chars = line.chars();
        let mut i = 0;
        for c in chars {
            if c == '1' {
                one_count[i] += 1;
            }
            i += 1;
        }
    }
    let oxygen_rating = get_ox_rating(one_count[0] as u32, numbers.clone(), 0)[0].clone();

    for i in oxygen_rating {
        if i == 1 {
            oxygen_generator_rating += "1";
        } else {
            oxygen_generator_rating += "0";
        }
    }
    println!("{}\n", oxygen_generator_rating);

    let co2_rating = get_co2_rating(one_count[0] as u32, numbers.clone(), 0)[0].clone();

    for i in co2_rating {
        if i == 1 {
            co2_scrubber_rating += "1";
        } else {
            co2_scrubber_rating += "0";
        }
    }

    println!("{}\n", co2_scrubber_rating);

    print!(
        "{}\n",
        isize::from_str_radix(oxygen_generator_rating.as_str(), 2).unwrap()
            * isize::from_str_radix(co2_scrubber_rating.as_str(), 2).unwrap()
    );
}

fn get_ox_rating(common: u32, numbers: Vec<Vec<u32>>, i: usize) -> Vec<Vec<u32>> {
    if numbers.len() == 1 {
        return numbers;
    }

    let mut one_count = 0;
    let mut new_vec: Vec<Vec<u32>> = Vec::new();

    let mut val = 0;
    if common >= (numbers.len() as f32 / 2.0).ceil() as u32 {
        val = 1;
    }

    for n in numbers.clone() {
        if n[i] == val {
            new_vec.push(n.clone());
            if i + 1 < NB_CHAR && n[i + 1] == 1 {
                one_count += 1;
            }
        }
    }

    get_ox_rating(one_count, new_vec, i + 1)
}

fn get_co2_rating(common: u32, numbers: Vec<Vec<u32>>, i: usize) -> Vec<Vec<u32>> {
    if numbers.len() == 1 {
        return numbers;
    }

    let mut one_count = 0;
    let mut new_vec: Vec<Vec<u32>> = Vec::new();

    let mut val = 0;
    if common < (numbers.len() as f32 / 2.0).ceil() as u32 {
        val = 1;
    }

    for n in numbers.clone() {
        if n[i] == val {
            new_vec.push(n.clone());
            if i + 1 < NB_CHAR && n[i + 1] == 1 {
                one_count += 1;
            }
        }
    }

    get_co2_rating(one_count, new_vec, i + 1)
}
