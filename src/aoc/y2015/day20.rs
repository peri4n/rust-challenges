use crate::aoc::Solution;

pub const INPUT: &str = "29000000";

pub struct Day20 {
    presents: usize,
}

impl Solution for Day20 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let presents = input.trim().parse().unwrap();
        Day20 { presents }
    }

    fn fst(&self) -> u32 {
        let mut houses = vec![0; self.presents / 10];

        for i in 1..houses.len() {
            for j in (i..houses.len()).step_by(i) {
                houses[j] += i * 10;
            }
        }

        houses.iter().position(|&x| x >= self.presents).unwrap() as u32
    }

    fn snd(&self) -> u32 {
        let mut houses = vec![0; self.presents / 10];

        for i in 1..houses.len() {
            for j in (i..houses.len()).step_by(i).take(50) {
                houses[j] += i * 11;
            }
        }
        houses.iter().position(|&x| x >= self.presents).unwrap() as u32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solution() {
        let day = Day20::parse(INPUT);
        assert_eq!(day.fst(), 665280);
        assert_eq!(day.snd(), 705600);
    }
}
