use itertools::Itertools;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

type LocationId = usize;

pub struct Day1 {
    ids: Vec<(LocationId, LocationId)>,
}

impl Solution for Day1 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let ids = input
            .lines()
            .map(|l| {
                let s: Vec<_> = l.split("   ").collect();
                (
                    s[0].parse::<LocationId>().unwrap(),
                    s[1].parse::<LocationId>().unwrap(),
                )
            })
            .collect();
        Day1 { ids }
    }

    fn fst(&self) -> usize {
        let first = self.ids.iter().map(|ids| ids.0).sorted();
        let second = self.ids.iter().map(|ids| ids.1).sorted();

        first.zip(second).map(|pair| pair.0.abs_diff(pair.1)).sum()
    }

    fn snd(&self) -> usize {
        let counts = self.ids.iter().map(|ids| ids.1).counts();

        self.ids
            .iter()
            .map(|ids| ids.0 * counts.get(&ids.0).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 2166959);
        assert_eq!(day.snd(), 23741109);
    }
}
