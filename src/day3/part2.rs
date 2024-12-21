use std::fs::read_to_string;
use std::path::Path;
use regex::Regex;

fn parse_line(line: &str, sums: &mut i64) {
    let MUL = r"mul\(".to_owned();
    let DIGIT = r"(?P<first>\d+)";
    let DIGIT2 = r"(?P<second>\d+)";
    let SEP = r",";
    let MUL_END = r"\)";

    let DO = r"do\(\)";
    let start_re = Regex::new(&DO).unwrap();

    let DONT = r"don't\(\)";
    let end_re = Regex::new(&DONT).unwrap();

    let pat = MUL + DIGIT + SEP + DIGIT2 + MUL_END;
    let re = Regex::new(&pat).unwrap();

    let mut i: usize = 0;
    let mut is_valid: bool = true;
    let mut cur_start: usize;
    let mut cur_end: usize;
    let len = line.len();
    while i < len {
        if is_valid {
            cur_end = match end_re.find_at(line, i) {
                Some(x) => x.start(),
                None => len
            };

            let (first, second) = match re.captures_at(line, i) {
                Some(m) => {
                    let f = m.get(0).unwrap();
                    if f.start() > cur_end {
                        i = cur_end;
                        is_valid = false;
                        (0, 0)
                    } else {
                        i = f.end();
                        (m["first"].parse().unwrap(), m["second"].parse().unwrap())
                    }
                },
                None => {
                    i = cur_end;
                    is_valid = false;
                    (0, 0)
                }
            };
            
            *sums += first * second;
        } else {
            cur_start = match start_re.find_at(line, i) {
                Some(x) => x.end(),
                None => len
            };

            i = cur_start;
            is_valid = true;
        }
    }
}

pub fn solve(filename: &str) -> i64 {
    let mut sum: i64 = 0;
    let path = Path::new(filename);
    let _disp = path.display();
    let buf: String = read_to_string(path).expect("Cannot read from file: {_disp}");
    parse_line(&buf, &mut sum);

    sum
}

