use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_line(line: &str, sums: &mut i64) {
    let MUL = r"mul\(".to_owned();
    let DIGIT = r"(?P<first>\d+)";
    let DIGIT2 = r"(?P<second>\d+)";
    let SEP = r",";
    let MUL_END = r"\)";

    let pat = MUL + DIGIT + SEP + DIGIT2 + MUL_END;
    let re = Regex::new(&pat).unwrap();
    for (_, [first, second]) in re.captures_iter(line).map(|c| c.extract()) {
        let first:i64 = first.parse().unwrap();
        let second:i64 = second.parse().unwrap();
        *sums += first * second;
        println!("{first}, {second}");
    }
}

pub fn solve(filename: &str) -> i64 {
    let mut sum: i64 = 0;
    let path = Path::new(filename);
    let _disp = path.display();
    let file = File::open(&path).expect("Cannot open file: {_disp}");
    let file = BufReader::new(file);

    for line in file.lines() {
        let valid_line = line.expect("Error reading file");
        parse_line(&valid_line, &mut sum);
    }

    sum
}
