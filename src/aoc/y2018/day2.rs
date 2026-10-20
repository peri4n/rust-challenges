use itertools::Itertools;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day2.txt");

pub struct Day2 {
    ids: Vec<String>,
}

impl Solution for Day2 {
    type Fst = i32;
    type Snd = String;

    fn parse(input: &str) -> Self {
        let ids = input.lines().map(String::from).collect();
        Day2 { ids }
    }

    fn fst(&self) -> i32 {
        let counts = self
            .ids
            .iter()
            .map(|id| categorize(id))
            .fold((0, 0), |acc, (two, three)| {
                (acc.0 + two as i32, acc.1 + three as i32)
            });

        counts.0 * counts.1
    }

    fn snd(&self) -> String {
        for (l1, l2) in self.ids.iter().tuple_combinations() {
            if let Some(res) = similar(l1, l2) {
                return res;
            }
        }

        panic!("Should not happen")
    }
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
    fn solution() {
        let day = Day2::parse(INPUT);
        assert_eq!(day.fst(), 5368);
        assert_eq!(day.snd(), String::from("cvgywxqubnuaefmsljdrpfzyi"));
    }
}
