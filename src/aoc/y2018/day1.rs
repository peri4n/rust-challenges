use std::collections::HashSet;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

pub struct Day1 {
    changes: Vec<i32>,
}

impl Solution for Day1 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let changes = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
        Day1 { changes }
    }

    fn fst(&self) -> i32 {
        self.changes.iter().sum()
    }

    fn snd(&self) -> i32 {
        let mut seen = HashSet::new();
        seen.insert(0);
        let mut frequency = 0;
        for f in self.changes.iter().cycle() {
            frequency += f;

            if !seen.insert(frequency) {
                return frequency;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 427);
        assert_eq!(day.snd(), 341);
    }
}
