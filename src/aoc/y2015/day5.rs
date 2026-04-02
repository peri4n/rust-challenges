use std::{collections::HashMap, fs, str};

const INPUT_FILE: &str = "src/aoc/y2015/day5.txt";

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn input() -> Vec<String> {
    fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

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

pub fn day5_fst() -> usize {
    input().iter().filter(|&l| validate_fst(l)).count()
}

pub fn day5_snd() -> usize {
    input().iter().filter(|&l| validate_snd(l)).count()
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
    fn solution_fst() {
        assert_eq!(day5_fst(), 258);
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
    fn solution_snd() {
        assert_eq!(day5_snd(), 53);
    }
}
