use std::{collections::HashSet, fs};

const INPUT_FILE: &str = "src/aoc/y2015/day19.txt";

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
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

fn input() -> (Vec<Rule>, String) {
    parse_input(fs::read_to_string(INPUT_FILE).unwrap().as_str())
}

pub fn day19_fst() -> i32 {
    let (rules, start_molecule) = input();

    let mut molecules = HashSet::new();

    for rule in rules {
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

pub fn day19_snd() -> i32 {
    let (rules, mut start_molecule) = input();

    let mut steps = 0;

    while start_molecule != "e" {
        let mut idx = 0;
        let mut found = true;
        for rule in rules.iter() {
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_day19_fst() {
        assert_eq!(day19_fst(), 535);
    }

    #[test]
    fn test_day19_snd() {
        assert_eq!(day19_snd(), 212);
    }
}
