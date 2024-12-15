use std::io;
use std::vec::Vec;

fn is_safe(arr: Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = Vec::new();

    for i in 0..arr.len() {
        let v1 = arr[0];
        let v2 = arr[i];

        let diff = v2 - v1;
        diffs.push(diff);
    }

    println!("is_safe: Array: {arr:?}");
    println!("is_safe: diffs: {diffs:?}");
    /*
    let mut dir:i8 = 0;
    for i in 1..diffs.len() {
        if diffs[i].abs() > 3 || diffs[i] == 0 {
            println!("=> change flaw at: {i}, arr[i={i}] = {}", arr[i]);
            flaws += 1;
            if flaws > 1 {
                return false;
            }
            if i < diffs.len() - 1 {
                if diffs[i] == 0 && diffs[i + 1] == 0 {
                    println!("--> failed at: {i}, arr[i={i}] = {}", arr[i]);
                    return false;
                }

                if diffs[i].abs() > 3 {
                    let combined_diff = diffs[i + 1] + diffs[i];
                    if combined_diff.abs() > 3 {
                        println!("--> failed at: {i}, arr[i={i}] = {}", arr[i]);
                        return false;
                    }
                }
            }
        } else {
            let cur_dir:i8 = diffs[i].clamp(-1, 1).try_into().unwrap();
            if dir == 0 {
                dir = cur_dir;
            } else if cur_dir != dir {
                flaws += 1;
                println!("=> direction flaw at: {i}, arr[i={i}] = {}", arr[i]);
                if flaws > 1 {
                    return false;
                } else {
                    dir = cur_dir;
                }
            }
        }
    }
    */
    return true;
}

fn main() {
    let lines = io::stdin().lines();
    let mut safe:u32 = 0;
    for line in lines {
        let mut arr: Vec<i32> = Vec::new();
        if line.is_err() {
            println!("Error for line");
            break;
        } else {
            let s = line.unwrap();
            let items: Vec<&str> = s.trim().split(' ').collect();
            for digit in items {
                if digit != "" {
                    arr.push(String::from(digit).trim().parse().expect("Not an integer"));
                }
            }
        }

        if is_safe(arr) {
            println!("==> SAFE");
            safe += 1;
        } else {
            println!("==> UNSAFE");
        }
    }

    println!("safe = {safe}");
}
