use std::io;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let mut freq = HashMap::new();
    let mut freq_left = HashMap::new();
    let lines = io::stdin().lines();

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
    }

    let mut sum:u32 = 0;
    for (id, frequency) in &freq {
        let repeat = match freq_left.get(&id) {
            Some(r) => *r,
            None => 0
        };

        sum += id * frequency * repeat;
    }

    println!("Sum is: {sum}");
}
