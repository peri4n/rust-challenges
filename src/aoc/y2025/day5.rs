use std::{fs, ops::RangeInclusive};

use nom::{
    IResult,
    character::complete::{newline, u64},
    multi::{fold_many1, separated_list1},
    sequence::terminated,
};
use range_set::RangeSet;

const INPUT_FILE: &str = "src/aoc/y2025/day5.txt";

fn input() -> Setup {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    parse_setup(&content).expect("Failed to parse input").1
}

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
pub fn day5_fst() -> usize {
    input().count_fresh()
}

pub fn day5_snd() -> usize {
    let setup = input();
    setup.ranges.len()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day5_fst(), 674);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day5_snd(), 352509891817881);
    }
}
