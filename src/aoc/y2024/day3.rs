use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "src/aoc/y2024/day3.txt";

fn day3_fst() -> i32 {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read input file");

    let re = Regex::new(r#"mul\((?<op1>\d+),(?<op2>\d+)\)"#).unwrap();

    let mut sum = 0;
    for captures in re.captures_iter(&content) {
        sum += captures["op1"].parse::<i32>().unwrap() * captures["op2"].parse::<i32>().unwrap();
    }

    sum
}

fn day3_snd() -> i32 {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read input file");

    let re = Regex::new(r#"(mul\((?<op1>\d+),(?<op2>\d+)\))|(do\(\))|don't\(\)"#).unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for captures in re.captures_iter(&content) {
        if captures.get(0).unwrap().as_str().starts_with("don't()") {
            enabled = false;
            continue;
        }

        if captures.get(0).unwrap().as_str().starts_with("do()") {
            enabled = true;
            continue;
        }

        if enabled {
            sum +=
                captures["op1"].parse::<i32>().unwrap() * captures["op2"].parse::<i32>().unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day3_fst(), 174561379);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day3_snd(), 106921067);
    }
}
