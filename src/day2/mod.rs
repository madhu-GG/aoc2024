pub mod part1;
pub mod part2;

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


