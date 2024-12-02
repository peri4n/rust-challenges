use std::fs;

use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2024/day1.txt";

type LocationId = usize;

fn input() -> Vec<(LocationId, LocationId)> {
    fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file")
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split("   ").collect();
            (
                s[0].parse::<LocationId>().unwrap(),
                s[1].parse::<LocationId>().unwrap(),
            )
        })
        .collect()
}

fn day1_fst() -> usize {
    let ids = input();
    let first = ids.iter().map(|ids| ids.0).sorted();
    let second = ids.iter().map(|ids| ids.1).sorted();

    first.zip(second).map(|pair| pair.0.abs_diff(pair.1)).sum()
}

fn day1_snd() -> usize {
    let ids = input();
    let counts = ids.iter().map(|ids| ids.1).counts();

    ids.iter()
        .map(|ids| ids.0 as usize * counts.get(&ids.0).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 2166959);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 23741109);
    }
}
