use std::collections::HashSet;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day19.txt");

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

pub struct Day19 {
    rules: Vec<Rule>,
    start_molecule: String,
}

fn parse_input(input: &str) -> (Vec<Rule>, String) {
    let lines = input.lines();

    let mut rules = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains("=>") {
            let mut parts = line.split(" => ");
            rules.push(Rule {
                from: parts.next().unwrap().to_string(),
                to: parts.next().unwrap().to_string(),
            });
        } else {
            return (rules, line.to_string());
        }
    }

    (rules, "".to_string())
}

impl Solution for Day19 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let (rules, start_molecule) = parse_input(input);
        Day19 { rules, start_molecule }
    }

    fn fst(&self) -> i32 {
        let start_molecule = &self.start_molecule;

        let mut molecules = HashSet::new();

        for rule in &self.rules {
            let mut idx = 0;
            while let Some(i) = start_molecule[idx..].find(&rule.from) {
                let match_in_start_molecule = i + idx;
                let mut new_molecule = start_molecule.clone();
                new_molecule.replace_range(
                    match_in_start_molecule..match_in_start_molecule + rule.from.len(),
                    &rule.to,
                );
                molecules.insert(new_molecule);
                idx = match_in_start_molecule + 1;
            }
        }

        molecules.len() as i32
    }

    fn snd(&self) -> i32 {
        let mut start_molecule = self.start_molecule.clone();

        let mut steps = 0;

        while start_molecule != "e" {
            let mut idx = 0;
            let mut found = true;
            for rule in self.rules.iter() {
                while let Some(i) = start_molecule[idx..].find(&rule.to) {
                    let match_in_start_molecule = i + idx;
                    start_molecule.replace_range(
                        match_in_start_molecule..match_in_start_molecule + rule.to.len(),
                        &rule.from,
                    );
                    steps += 1;
                    idx = match_in_start_molecule + 1;
                    found = false;
                }
            }

            if found {
                break;
            }
        }

        steps
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day19::parse(INPUT);
        assert_eq!(day.fst(), 535);
        assert_eq!(day.snd(), 212);
    }
}
