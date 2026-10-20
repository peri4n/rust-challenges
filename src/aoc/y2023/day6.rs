use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, u64};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day6.txt");

pub struct Day6 {
    problem: Problem,
}

impl Solution for Day6 {
    type Fst = u64;
    type Snd = u64;

    fn parse(input: &str) -> Self {
        let problem = parse_problem(input).unwrap().1;
        Day6 { problem }
    }

    fn fst(&self) -> u64 {
        self.problem.solution1()
    }

    fn snd(&self) -> u64 {
        // the spaces between the numbers are ignored: it's a single race
        let problem = Problem {
            records: vec![concat(&self.problem.records)],
            times: vec![concat(&self.problem.times)],
        };

        problem.solution1()
    }
}

fn concat(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

struct Problem {
    records: Vec<u64>,
    times: Vec<u64>,
}

impl Problem {
    pub fn solution1(&self) -> u64 {
        let mut res = 1;
        for i in 0..self.times.len() {
            res *= ways_to_beat(self.times[i], self.records[i]);
        }
        res
    }
}

fn ways_to_beat(distance: u64, record: u64) -> u64 {
    let mut ways = 0;
    for velocity in 1..distance {
        let rest_time = distance - velocity;
        if rest_time * velocity > record {
            ways += 1;
        }
    }
    ways
}

fn parse_problem(text: &str) -> IResult<&str, Problem> {
    terminated(parse_times, newline)
        .and(terminated(parse_records, newline))
        .map(|(times, records)| Problem { records, times })
        .parse(text)
}

fn parse_times(text: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("Time:").and(many1(char(' '))),
        separated_list1(many1(char(' ')), u64),
    )(text)
}

fn parse_records(text: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("Distance:").and(many1(char(' '))),
        separated_list1(many1(char(' ')), u64),
    )(text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let problem = Problem {
            times: vec![7, 15, 30],
            records: vec![9, 40, 200],
        };
        assert_eq!(problem.solution1(), 288);
    }

    #[test]
    fn solution() {
        let day = Day6::parse(INPUT);
        assert_eq!(day.fst(), 32076);
        assert_eq!(day.snd(), 34278221);
    }
}
