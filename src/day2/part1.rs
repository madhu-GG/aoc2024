
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

fn is_safe(digits: &Vec<u32>) -> bool {
    let mut safe: bool = true;
    let mut previous: i64 = -1;
    let mut prev_diff: i64 = 0;

    for digit in digits {
        if previous == -1 {
            previous = (*digit) as i64;
        } else {
            let diff: i64 = *digit as i64 - previous;
            if diff == 0 || diff.abs() > 3 {
                safe = false;
                break;
            }

            if (prev_diff < 0 && diff > 0)
                || (prev_diff > 0 && diff < 0) {
                safe = false;
                break;
            }

            prev_diff = diff;
            previous = (*digit) as i64;
        }
    }

    safe
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
