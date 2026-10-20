use std::ops::RangeInclusive;

use nom::{
    IResult,
    character::complete::{newline, u64},
    multi::{fold_many1, separated_list1},
    sequence::terminated,
};
use range_set::RangeSet;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day5.txt");

fn parse_setup(content: &str) -> IResult<&str, Setup> {
    let (rest, ranges) = parse_ranges(content)?;
    let (rest, _) = newline(rest)?;
    let (rest, ids) = parse_ids(rest)?;
    Ok((rest, Setup { ranges, fresh_ids: ids }))
}

fn parse_ids(rest: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(newline, u64)(rest)
}

fn parse_ranges(content: &str) -> IResult<&str, RangeSet<[RangeInclusive<u64>; 2]>> {
    fold_many1(
        terminated(parse_range, newline),
        RangeSet::new,
        |mut acc: RangeSet<_>, item| {
            acc.insert_range(item);
            acc
        },
    )(content)
}

fn parse_range(rest: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (rest, start) = u64(rest)?;
    let (rest, _) = nom::bytes::complete::tag("-")(rest)?;
    let (rest, end) = u64(rest)?;
    Ok((rest, start..=end))
}

struct Setup {
    ranges: RangeSet<[RangeInclusive<u64>; 2]>,
    fresh_ids: Vec<u64>,
}

impl Setup {
    pub fn count_fresh(&self) -> usize {
        self.fresh_ids
            .iter()
            .filter(|&id| self.ranges.contains(*id))
            .count()
    }
}

pub struct Day5 {
    setup: Setup,
}

impl Solution for Day5 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let setup = parse_setup(input).expect("Failed to parse input").1;
        Day5 { setup }
    }

    fn fst(&self) -> usize {
        self.setup.count_fresh()
    }

    fn snd(&self) -> usize {
        self.setup.ranges.len()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day5::parse(INPUT);
        assert_eq!(day.fst(), 674);
        assert_eq!(day.snd(), 352509891817881);
    }
}
