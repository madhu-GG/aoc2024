use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;

use crate::day2::parse_line;

fn dir(diff: i32) -> i32 {
    match diff.cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0
    }
}

fn make_safe(digits: &Vec<u32>, stack: &mut Vec<usize>, index: usize) -> u32 {
    if index >= digits.len() {
        eprintln!("Final Sequence: {stack:?}");
        return 0;
    }

    let prev_index = stack[stack.len() - 1];
    let diff = digits[index] as i32 - digits[prev_index] as i32;
    let prev_diff: i32;
    let prev_prev_index: usize;
    if stack.len() >= 2 {
        prev_prev_index = stack[stack.len() - 2];
        prev_diff = digits[prev_index] as i32 - digits[prev_prev_index] as i32;
    } else {
        prev_diff = diff;
        prev_prev_index = digits.len();
    }

    let cdir = dir(diff);
    let pdir = dir(prev_diff);

    eprintln!("i={index}: {digits:?} stack: {stack:?}, value: {0}, diff: {diff}", digits[index]);
    if diff == 0 || diff.abs() > 3 || cdir != pdir {
        let score1: u32;
        // try to skip previous item 
        if stack.len() > 0 {
            let mut new_stack = stack.clone();
            let new_index: usize;
            let removed = new_stack.pop().expect("No items found in stack");
            if new_stack.len() == 0 {
                new_stack.push(index);
                new_index = index + 1;
            } else {
                new_index = index;
            }

            eprintln!("\t => skip digits[{0}] = {1}", removed, digits[removed]);
            score1 = 1 + make_safe(&digits, &mut new_stack, new_index);
        } else {
            score1 = 100;
        }

        // try to ignore current item
        eprintln!("\t => skip digits[{index}] = value: {0}", digits[index]);
        let score2 = 1 + make_safe(&digits, stack, index + 1);

        // skip prev to prev item or current item
        let score3: u32;
        if cdir != pdir && (prev_prev_index == 0 || index == digits.len() - 1) {
            if prev_prev_index == 0 {
                let mut new_stack = stack.clone();
                let removed = new_stack.pop().expect("1st: No items found in stack");
                let removed_2 = new_stack.pop().expect("2nd: No items found in stack");
                new_stack.push(removed);
                new_stack.push(index);
                let new_diff = digits[index] as i32 - digits[removed] as i32;
                if new_diff.abs() > 3 {
                    score3 = 100;
                } else {
                    eprintln!("\t => skip digits[{0}] = {1}", removed_2, digits[removed_2]);
                    score3 = 1 + make_safe(&digits, &mut new_stack, index + 1);
                }
            } else {
                score3 = 1;
            }
        } else {
            score3 = 100;
        }

        eprintln!("Score3 is {score3}");
        let mut min_score:u32 = score3;
        if score2 < min_score {
            min_score = score2;
        }

        if score1 < min_score {
            min_score = score1;
        }

        //eprintln!("Returned to i={index}, score: {min_score}");
        return min_score;
    } else {
        stack.push(index);
        return make_safe(&digits, stack, index + 1);
    }
}

fn is_safe(digits: &Vec<u32>) -> bool {
    let mut stack: Vec<usize> = Vec::new();

    stack.push(0);
    let num_changes = make_safe(&digits, &mut stack, 1);
    eprintln!("--- {digits:?} safe after {num_changes} changes. --- ");
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
