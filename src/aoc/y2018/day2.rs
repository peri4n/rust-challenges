use std::fs;

use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2018/day2.txt";

pub fn day2_fst() -> i32 {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to find file");
    let counts = content
        .lines()
        .map(categorize)
        .fold((0, 0), |acc, (two, three)| {
            (acc.0 + two as i32, acc.1 + three as i32)
        });

    counts.0 * counts.1
}
pub fn day2_snd() -> String {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to find file");

    let lines = content.lines().collect::<Vec<_>>();

    for (l1, l2) in lines.iter().tuple_combinations() {
        if let Some(res) = similar(l1, l2) {
            return res;
        }
    }

    panic!("Should not happen")
}

fn categorize(id: &str) -> (bool, bool) {
    let mut chars = [0; 26];
    for c in id.bytes() {
        chars[(c - b'a') as usize] += 1;
    }

    let mut found_two = false;
    let mut found_three = false;
    for count in chars {
        if count == 2 {
            found_two = true;
            continue;
        }
        if count == 3 {
            found_three = true;
        }
    }

    (found_two, found_three)
}

fn similar(id1: &str, id2: &str) -> Option<String> {
    let common: String = id1
        .chars()
        .zip(id2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect();

    if id1.len() - common.len() == 1 {
        Some(common)
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day2_fst(), 5368);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), String::from("cvgywxqubnuaefmsljdrpfzyi"));
    }
}
