use std::fs;

const INPUT_FILE: &str = "src/aoc/y2023/day1.txt";

fn input() -> Vec<String> {
    fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

pub fn day1_fst() -> u32 {
    input().iter().map(|line| count(line)).sum()
}

pub fn day1_snd() -> u32 {
    input().iter().map(|line| count(&replace(line))).sum()
}

fn count(line: &str) -> u32 {
    let mut result: Vec<u8> = Vec::new();
    for b in line.bytes() {
        if b.is_ascii_digit() {
            result.push(b & 15); // coolest trick ever
        }
    }
    *result.first().unwrap() as u32 * 10 + *result.last().unwrap() as u32
}

fn replace(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "9e")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 54388);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 53515);
    }
}
