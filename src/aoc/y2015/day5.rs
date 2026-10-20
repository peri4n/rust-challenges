use std::{collections::HashMap, str};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day5.txt");

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn validate_fst(str: &str) -> bool {
    has_no_forbidden_digram(str)
        && contains_a_letter_twice(str)
        && contains_at_least_three_vowels(str)
}

fn validate_snd(str: &str) -> bool {
    has_repeating_digram(str) && has_repeat_with_gap(str)
}

fn has_repeating_digram(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut first_seen = HashMap::with_capacity(bytes.len());

    bytes
        .windows(2)
        .enumerate()
        .any(|(i, pair)| match first_seen.get(pair) {
            Some(&prev_i) if i >= prev_i + 2 => true,
            Some(_) => false,
            None => {
                first_seen.insert(pair, i);
                false
            }
        })
}

fn has_repeat_with_gap(characters: &str) -> bool {
    characters.as_bytes().windows(3).any(|w| w[0] == w[2])
}

fn has_no_forbidden_digram(str: &str) -> bool {
    !str.contains("ab") && !str.contains("cd") && !str.contains("pq") && !str.contains("xy")
}

fn contains_a_letter_twice(str: &str) -> bool {
    str.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn contains_at_least_three_vowels(str: &str) -> bool {
    str.chars().filter(|c| VOWELS.contains(c)).take(3).count() == 3
}

pub struct Day5 {
    lines: Vec<String>,
}

impl Solution for Day5 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day5 {
            lines: input.lines().map(|l| l.to_owned()).collect(),
        }
    }

    fn fst(&self) -> usize {
        self.lines.iter().filter(|&l| validate_fst(l)).count()
    }

    fn snd(&self) -> usize {
        self.lines.iter().filter(|&l| validate_snd(l)).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_a_letter_twice_examples() {
        assert!(contains_a_letter_twice("crximadpvdaccrsm"));
    }

    #[test]
    fn contains_two_vowels_examples() {
        assert!(contains_at_least_three_vowels("crximadpvdaccrsm"));
    }

    #[test]
    fn has_no_forbidden_digram_examples() {
        assert!(has_no_forbidden_digram("crximadpvdaccrsm"));
    }

    #[test]
    fn has_repeating_digram_examples() {
        assert!(has_repeating_digram("qjhvhtzxzqqjkmpb"));
        assert!(has_repeating_digram("uurcxstgmygtbstg"));
        assert!(has_repeating_digram("xxyxx"));
        assert!(!has_repeating_digram("ieodomkazucvgmuy"));
    }

    #[test]
    fn has_repeat_with_gap_examples() {
        assert!(has_repeat_with_gap("qjhvhtzxzqqjkmpb"));
        assert!(has_repeat_with_gap("xxyxx"));
        assert!(!has_repeat_with_gap("uurcxstgmygtbstg"));
        assert!(has_repeat_with_gap("ieodomkazucvgmuy"));
    }

    #[test]
    fn solution() {
        let day = Day5::parse(INPUT);
        assert_eq!(day.fst(), 258);
        assert_eq!(day.snd(), 53);
    }
}
