use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

pub struct Day1 {
    digits: Vec<u32>,
}

impl Solution for Day1 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let digits = input.chars().filter_map(|c| c.to_digit(10)).collect();
        Day1 { digits }
    }

    fn fst(&self) -> u32 {
        let digits = &self.digits;
        let mut sum = 0;

        for i in 0..digits.len() {
            if digits[i] == digits[(i + 1) % digits.len()] {
                sum += digits[i];
            }
        }
        sum
    }

    fn snd(&self) -> u32 {
        let digits = &self.digits;
        let mut sum = 0;
        let step = digits.len() / 2;

        for i in 0..digits.len() {
            if digits[i] == digits[(i + step) % digits.len()] {
                sum += digits[i];
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 1102);
        assert_eq!(day.snd(), 1076);
    }
}
