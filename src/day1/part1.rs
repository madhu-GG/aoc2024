use std::iter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::path::Path;

fn parse_line(line: &str, first: &mut Vec<u32>, second: &mut Vec<u32>) {
    let digits: Vec<&str> = line.trim().split(' ').collect();
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

    let a = clean_digits[0].trim().parse().expect("{clean_digits[0]} Not an unsigned integer");
    let b = clean_digits[1].trim().parse().expect("{clean_digits[1]} Not an unsigned integer");
    first.push(a);
    second.push(b);
}

pub fn solve(filename: &str) -> u32 {
    let path = Path::new(filename);
    let _disp = path.display();
    let file = File::open(&path).expect("Cannot open file: {_disp}");
    let file = BufReader::new(file);
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    for line in file.lines() {
        let line = line.expect("Cannot read line from file");
        parse_line(&line, &mut first, &mut second);
    }

    let mut sum:u32 = 0;
    for (a, b) in iter::zip(first, second) {
        sum += a.abs_diff(b);
    }

    sum
}
