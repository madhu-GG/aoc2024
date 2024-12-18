use std::env::args;
use std::vec::Vec;
use aoc2024::day2::part1;
use aoc2024::day2::part2;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage: {0} INPUT-FILE (part1|part2)", args[0]);
        return;
    }

    let filename: &String = &args[1];
    let part: &String = &args[2];
    let result:u32 = match part.as_str() {
        "part1" => part1::solve(&filename),
        "part2" => part2::solve(&filename),
        _ => 0
    };

    println!("Answer: {result}");
}
