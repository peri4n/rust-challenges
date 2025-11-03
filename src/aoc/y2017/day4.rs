use std::{collections::HashSet, fs};

use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2017/day4.txt";

pub fn day4_fst() -> usize {
    fs::read_to_string(INPUT_FILE)
        .expect("Failed to read input file: 2017/day4.txt")
        .lines()
        .filter(is_valid)
        .count()
}

pub fn day4_snd() -> usize {
    fs::read_to_string(INPUT_FILE)
        .expect("Failed to read input file: 2017/day4.txt")
        .lines()
        .filter(is_valid2)
        .count()
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
    fn solution_fst() {
        assert_eq!(day4_fst(), 466);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day4_snd(), 251);
    }
}
