use std::{collections::HashMap, fs, str};

const INPUT_FILE: &str = "src/aoc/y2015/day5.txt";

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
    has_repeating_digram(&str) && has_repeat_with_gap(&str)
}

fn has_repeating_digram(characters: &str) -> bool {
    let characters: Vec<_> = characters.chars().collect();
    let mut occurences = HashMap::with_capacity(characters.len());

    for i in 1..characters.len() {
        occurences
            .entry((characters[i - 1], characters[i]))
            .and_modify(|e: &mut Vec<_>| e.push(i))
            .or_insert(vec![i]);
    }

    for (_, occ) in occurences {
        for i in 1..occ.len() {
            if occ[i] - occ[i - 1] >= 2 {
                return true;
            }
        }
    }

    false
}

fn has_repeat_with_gap(characters: &str) -> bool {
    let characters: Vec<_> = characters.chars().collect();

    for i in 2..characters.len() {
        if characters[i] == characters[i - 2] {
            return true;
        }
    }

    false
}

fn has_no_forbidden_digram(str: &str) -> bool {
    !str.contains("ab") && !str.contains("cd") && !str.contains("pq") && !str.contains("xy")
}

fn contains_a_letter_twice(str: &str) -> bool {
    let characters: Vec<char> = str.chars().collect();

    for i in 1..characters.len() {
        if characters[i] == characters[i - 1] {
            return true;
        }
    }

    false
}

fn contains_at_least_three_vowels(str: &str) -> bool {
    let mut count = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    for c in str.chars() {
        if vowels.contains(&c) {
            count += 1;
        }

        if count >= 3 {
            return true;
        }
    }
    false
}

fn day5_fst() -> usize {
    input().iter().filter(|&l| validate_fst(l)).count()
}

fn day5_snd() -> usize {
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
