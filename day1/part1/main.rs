use std::io;
use std::iter;
use std::vec::Vec;

fn main() {
    let lines = io::stdin().lines();
    let mut first:Vec<u32> = Vec::new();
    let mut second:Vec<u32> = Vec::new();
    for line in lines {
        let a:u32;
        let b:u32;
        if line.is_err() {
            break;
        } else {
            let s = line.unwrap();
            let digits: Vec<&str> = s.trim().split(' ').collect();
            let mut clean_digits: Vec<&str> = Vec::new();
            for digit in digits {
                if digit != "" {
                    clean_digits.push(digit);
                }
            }
            if clean_digits.len() != 2 {
                println!("{clean_digits:?}");
                println!("Invalid line {s}");
                return;
            }

            a = clean_digits[0].trim().parse().expect("{clean_digits[0]} Not an unsigned integer");
            b = clean_digits[1].trim().parse().expect("{clean_digits[1]} Not an unsigned integer");

            first.push(a);
            second.push(b);
        }
    }

    first.sort();
    second.sort();

    let mut sum:u32 = 0;
    for (a, b) in iter::zip(first, second) {
        sum += a.abs_diff(b);
    }

    println!("Sum is: {sum}");
}
