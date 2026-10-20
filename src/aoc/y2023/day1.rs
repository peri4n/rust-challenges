use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

pub struct Day1 {
    lines: Vec<String>,
}

impl Solution for Day1 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let lines = input.lines().map(|l| l.to_owned()).collect();
        Day1 { lines }
    }

    fn fst(&self) -> u32 {
        self.lines.iter().map(|line| count(line)).sum()
    }

    fn snd(&self) -> u32 {
        self.lines.iter().map(|line| count(&replace(line))).sum()
    }
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
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 54388);
        assert_eq!(day.snd(), 53515);
    }
}
