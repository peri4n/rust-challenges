use std::fs;

const INPUT_FILE: &str = "src/aoc/y2016/day1.txt";

fn input() -> String {
    fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file")
}

