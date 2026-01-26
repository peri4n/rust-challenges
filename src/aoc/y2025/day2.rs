use std::fs;

use nom::{
    IResult,
    character::complete::{char, u64},
    multi::separated_list1,
};

const INPUT_FILE: &str = "src/aoc/y2025/day2.txt";

fn input() -> Vec<Range> {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    parse_content(&content)
        .expect("Failed to parse input content")
        .1
}

fn parse_content(content: &str) -> IResult<&str, Vec<Range>> {
    fn parse_range(input: &str) -> IResult<&str, Range> {
        let (input, start) = u64(input)?;
        let (input, _) = char('-')(input)?;
        let (input, end) = u64(input)?;

        Ok((input, Range { start, end }))
    }

    separated_list1(char(','), parse_range)(content)
}

pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn sum_invalid(&self) -> u64 {
        let mut sum = 0;
        for num in self.start..=self.end {
            if Range::is_invalid(num) {
                sum += num;
            }
        }
        sum
    }

    pub fn sum_invalid2(&self) -> u64 {
        let mut sum = 0;
        for num in self.start..=self.end {
            if Range::is_invalid2(num) {
                sum += num;
            }
        }
        sum
    }

    fn is_invalid(num: u64) -> bool {
        let num_digigts = num.ilog10() as u32 + 1;

        if num_digigts % 2 == 1 {
            return false;
        }
        let lhs = num / 10u64.pow(num_digigts / 2);
        let rhs = num % 10u64.pow(num_digigts / 2);
        lhs == rhs
    }

    fn is_invalid2(num: u64) -> bool {
        let digits = num.ilog10() as usize + 1;

        // Try splitting into k equal parts where k >= 2
        for k in 2..=digits {
            if digits % k != 0 {
                continue;
            }

            let block_len = digits / k;
            let pow = 10u64.pow(block_len as u32);
            let mut temp = num;

            // Extract the last block; this will be the pattern
            let pattern = temp % pow;
            temp /= pow;

            let mut ok = true;

            // Compare all remaining blocks
            for _ in 1..k {
                if temp % pow != pattern {
                    ok = false;
                    break;
                }
                temp /= pow;
            }

            if ok {
                return true;
            }
        }

        false
    }
}

pub fn day2_fst() -> u64 {
    input().iter().map(|r| r.sum_invalid()).sum()
}

pub fn day2_snd() -> u64 {
    input().iter().map(|r| r.sum_invalid2()).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples_fst() {
        assert_eq!(Range { start: 11, end: 22 }.sum_invalid(), 11 + 22);
        assert_eq!(Range { start: 95, end: 115 }.sum_invalid(), 99);
        assert_eq!(Range { start: 998, end: 1012 }.sum_invalid(), 1010);
        assert_eq!(
            Range {
                start: 1188511880,
                end: 1188511890
            }
            .sum_invalid(),
            1188511885
        );
        assert_eq!(
            Range {
                start: 222220,
                end: 222225
            }
            .sum_invalid(),
            222222
        );
        assert_eq!(
            Range {
                start: 38593856,
                end: 38593862
            }
            .sum_invalid(),
            38593859
        );
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day2_fst(), 44854383294);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), 55647141923);
    }
}
