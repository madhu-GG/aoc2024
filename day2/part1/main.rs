use std::io;
use std::vec::Vec;

fn main() {
    let lines = io::stdin().lines();
    let mut safe:u32 = 0;
    for line in lines {
        let mut clean_digits: Vec<String> = Vec::new();
        if line.is_err() {
            break;
        } else {
            let s = line.unwrap();
            let digits: Vec<&str> = s.trim().split(' ').collect();
            for digit in digits {
                if digit != "" {
                    clean_digits.push(String::from(digit));
                }
            }
        }

        let mut is_safe: bool = true;
        let mut previous:i64 = -1;
        let mut prev_diff:i64 = 0;
        let mut a:i64;
        for digit in clean_digits {
            a = digit.trim().parse()
                    .expect("{digit} is not an integer");

            if previous == -1 {
                previous = a;
            } else {
                let diff:i64 = a - previous;
                if diff == 0 || diff.abs() > 3 {
                    is_safe = false;
                    break;
                }

                if (prev_diff < 0 && diff > 0)
                    || (prev_diff > 0 && diff < 0) {
                    is_safe = false;
                    break;
                }

                prev_diff = diff;
                previous = a;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    println!("safe = {safe}");
}
