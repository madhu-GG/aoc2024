use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn parse_line(line: &str, digits: &mut Vec<u32>) {
    let tokens: Vec<&str> = line.trim()
        .split(' ')
        .collect();

    for token in tokens {
        if token != "" {
            let digit: u32 = token.parse().unwrap();
            digits.push(digit);
        }
    }
}

fn make_safe(digits: &Vec<u32>, stack: &mut Vec<u32>, index: usize) -> u32 {
    if index >= digits.len() {
        return 0;
    }

    let diff = digits[index] as i32 - stack[stack.len() - 1] as i32;
    //println!("{digits:?} stack: {stack:?}, value: {0}, diff: {diff}", digits[index]);
    if diff == 0 || diff.abs() > 3 {
        let mut score1: u32 = 0;
        // try to remove previous item
        if stack.len() > 0 {
            let mut new_stack = stack.clone();
            let new_index: usize;
            new_stack.pop();
            if new_stack.len() == 0 {
                new_stack.push(digits[index]);
                new_index = index + 1;
            } else {
                new_index = index;
            }

            score1 = 1 + make_safe(&digits, &mut new_stack, new_index);
        }

        // try to ignore current item
        let score2 = 1 + make_safe(&digits, stack, index + 1);

        if score1 < score2 {
            return score1;
        } else {
            return score2;
        }
    } else {
        stack.push(digits[index]);
        return make_safe(&digits, stack, index + 1);
    }
}

fn is_safe(digits: &Vec<u32>) -> bool {
    let mut stack: Vec<u32> = Vec::new();

    stack.push(digits[0]);
    let num_changes = make_safe(&digits, &mut stack, 1);

    println!("{digits:?} safe after {num_changes} changes.");

    return num_changes <= 1;
}

pub fn solve(filename: &str) -> u32 {
    let path = Path::new(filename);
    let _disp = path.display();
    let file = File::open(&path).expect("Cannot open file: {_disp}");
    let file = BufReader::new(file);
    let mut digits: Vec<u32> = Vec::new();

    let mut safe_count:u32 = 0;
    for line in file.lines() {
        let line = line.expect("Cannot read line from file");
        digits.clear();
        parse_line(&line, &mut digits);
        if is_safe(&digits) {
            safe_count += 1;
        }
    }

    safe_count
 }
