use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::HashMap;
use std::path::Path;

fn parse_line(line: &str, freq: &mut HashMap<u32, u32>, freq_left: &mut HashMap<u32, u32>) {
    let digits: Vec<&str> = line
        .trim()
        .split(' ')
        .collect();

    let mut clean_digits: Vec<&str> = Vec::new();
    for digit in digits {
        if digit != "" {
            clean_digits.push(digit);
        }
    }
    
    if clean_digits.len() != 2 {
        println!("{clean_digits:?}");
        println!("Invalid line {line}");
        return;
    }

    let a = clean_digits[0]
                .trim()
                .parse()
                .expect("{clean_digits[0]} Not an unsigned integer");
    let b = clean_digits[1]
                .trim()
                .parse()
                .expect("{clean_digits[1]} Not an unsigned integer");

    let cur_freq_left = match freq_left.get(&a) {
        Some(f) => *f + 1,
        None => 1
    };

    freq_left.insert(a, cur_freq_left);

    if !freq.contains_key(&a) {
        freq.insert(a, 0);
    }

    let cur_freq = match freq.get(&b) {
        Some(f) => *f + 1,
        None => 1
    };

    freq.insert(b, cur_freq);
}

pub fn solve(filename: &str) -> u32 {
    let path = Path::new(filename);
    let _disp = path.display();
    let file = File::open(&path).expect("Cannot open file: {_disp}");
    let file = BufReader::new(file);
    let mut freq: HashMap<u32, u32> = HashMap::new();
    let mut freq_left: HashMap<u32, u32> = HashMap::new();

    for line in file.lines() {
        let line = line.expect("Cannot read line from file");
        parse_line(&line, &mut freq, &mut freq_left);
    }

    let mut sum:u32 = 0;
    for (id, frequency) in &freq {
        let repeat = match freq_left.get(&id) {
            Some(r) => *r,
            None => 0
        };

        sum += id * frequency * repeat;
    }

    sum
}