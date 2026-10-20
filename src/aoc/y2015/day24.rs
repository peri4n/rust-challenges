use itertools::Itertools;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day24.txt");

pub struct Day24 {
    weights: Vec<u64>,
}

impl Solution for Day24 {
    type Fst = u64;
    type Snd = u64;

    fn parse(input: &str) -> Self {
        let weights = input
            .lines()
            .map(|line| line.parse::<u64>().expect("Could not parse line as u32"))
            .collect();
        Day24 { weights }
    }

    fn fst(&self) -> u64 {
        let weights = &self.weights;
        let total_weight: u64 = weights.iter().sum();
        let target_weight = total_weight / 3;

        for size in 1..=weights.len() {
            let mut min_qe = None;

            for combo in weights.iter().combinations(size) {
                let sum: u64 = combo.iter().copied().sum();
                if sum == target_weight {
                    let qe: u64 = combo.iter().copied().product();
                    match min_qe {
                        Some(current_min) if qe < current_min => min_qe = Some(qe),
                        None => min_qe = Some(qe),
                        _ => {}
                    }
                }
            }

            if let Some(qe) = min_qe {
                return qe;
            }
        }

        0
    }

    fn snd(&self) -> u64 {
        let weights = &self.weights;
        let total_weight: u64 = weights.iter().sum();
        let target_weight = total_weight / 4;

        for size in 1..=weights.len() {
            let mut min_qe = None;

            for combo in weights.iter().combinations(size) {
                let sum: u64 = combo.iter().copied().sum();
                if sum == target_weight {
                    let qe: u64 = combo.iter().copied().product();
                    match min_qe {
                        Some(current_min) if qe < current_min => min_qe = Some(qe),
                        None => min_qe = Some(qe),
                        _ => {}
                    }
                }
            }

            if let Some(qe) = min_qe {
                return qe;
            }
        }

        0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day24::parse(INPUT);
        assert_eq!(day.fst(), 10723906903);
        assert_eq!(day.snd(), 74850409);
    }
}
