use std::env::args;
use aoc2024::day1::part1;
use aoc2024::day1::part2;

fn main() {
    let mut args = args();
    if args.len() != 3 {
        let prog_name: String = args.nth(0).expect("Invalid string");

        println!("Usage: {prog_name} INPUT-FILE (part1|part2)");
        return;
    }

    let filename: String = args.nth(1).expect("Invalid filename given at position 1");
    let part = args.nth(2).expect("Part is missing");
    let result:u32 = match part.as_str() {
        "part1" => part1::solve(&filename),
        "part2" => part2::solve(&filename),
        _ => 0
    };

    println!("Answer: {result}");
}
