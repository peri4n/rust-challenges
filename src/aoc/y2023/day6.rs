use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, u64};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;

pub fn day6_fst() -> u64 {
    let content = include_str!("../y2023/day6.txt");
    let problem = parse_problem(content).unwrap().1;

    problem.solution1()
}

pub fn day6_snd() -> u64 {
    let problem = Problem {
        records: vec![208158110501102],
        times: vec![44806572],
    };

    problem.solution1()
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
    fn solution_fst() {
        assert_eq!(day6_fst(), 32076);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day6_snd(), 34278221);
    }
}
