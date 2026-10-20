use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day4.txt");

pub struct Day4 {
    passphrases: Vec<String>,
}

impl Solution for Day4 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let passphrases = input.lines().map(String::from).collect();
        Day4 { passphrases }
    }

    fn fst(&self) -> usize {
        self.passphrases
            .iter()
            .map(String::as_str)
            .filter(is_valid)
            .count()
    }

    fn snd(&self) -> usize {
        self.passphrases
            .iter()
            .map(String::as_str)
            .filter(is_valid2)
            .count()
    }
}

fn is_valid(line: &&str) -> bool {
    let mut words_seen = HashSet::new();
    for word in line.split(' ') {
        if words_seen.contains(word) {
            return false;
        }

        words_seen.insert(word);
    }
    true
}

fn is_valid2(line: &&str) -> bool {
    let mut words_seen = HashSet::new();
    for word in line.split(' ') {
        let normalized_word = normalize(word);
        if words_seen.contains(&normalized_word) {
            return false;
        }

        words_seen.insert(normalized_word);
    }
    true
}

fn normalize(word: &str) -> String {
    word.chars().sorted().collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day4::parse(INPUT);
        assert_eq!(day.fst(), 466);
        assert_eq!(day.snd(), 251);
    }
}
