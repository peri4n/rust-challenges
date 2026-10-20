use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day3.txt");

pub struct Day3 {
    batteries: Vec<Vec<u8>>,
}

impl Solution for Day3 {
    type Fst = u32;
    type Snd = u128;

    fn parse(input: &str) -> Self {
        let batteries = input
            .lines()
            .map(|line| line.bytes().map(|b| b & 0b1111).collect())
            .collect();
        Day3 { batteries }
    }

    fn fst(&self) -> u32 {
        let mut sum = 0;
        for cells in &self.batteries {
            sum += largest_subsequence(cells, 2);
        }

        sum as u32
    }

    fn snd(&self) -> u128 {
        let mut sum = 0;
        for cells in &self.batteries {
            sum += largest_subsequence(cells, 12);
        }

        sum
    }
}

pub fn largest_subsequence(digits: &[u8], k: usize) -> u128 {
    if digits.len() < k {
        return 0;
    }

    let mut result: Vec<u8> = Vec::with_capacity(k);
    let mut to_pick = k;

    let mut i = 0;
    while to_pick > 0 {
        let remaining = digits.len() - i;
        let end = remaining - (to_pick - 1);

        // Find the maximum digit in digits[i .. i+end)
        let mut max_digit = 0u8;
        let mut max_pos = i;

        for pos in i..i + end {
            let d = digits[pos];
            if d > max_digit {
                max_digit = d;
                max_pos = pos;
                if d == 9 {
                    break;
                }
            }
        }

        result.push(max_digit);
        to_pick -= 1;
        i = max_pos + 1;
    }

    // Convert to number
    result.iter().fold(0u128, |acc, &d| acc * 10 + d as u128)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day3::parse(INPUT);
        assert_eq!(day.fst(), 17095);
        assert_eq!(day.snd(), 168794698570517);
    }
}
