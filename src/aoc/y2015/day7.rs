use std::collections::{HashMap, VecDeque};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, i32};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day7.txt");

// Circuit definition for part two (signal b overridden with the result of part one)
pub const INPUT_2: &str = include_str!("day7_2.txt");

#[derive(Debug)]
enum Op {
    Not {
        input: String,
        output: String,
    },
    Assign {
        input: String,
        value: i32,
    },
    And {
        input1: String,
        input2: String,
        output: String,
    },
    Or {
        input1: String,
        input2: String,
        output: String,
    },
    LShift {
        input: String,
        value: i32,
        output: String,
    },
    RShift {
        input: String,
        value: i32,
        output: String,
    },
    Alias {
        input: String,
        output: String,
    },
}

fn parse_not(line: &str) -> IResult<&str, Op> {
    let input = preceded(tag("NOT "), alpha1);
    let output = preceded(tag(" -> "), alpha1);
    input
        .and(output)
        .map(|(i, o): (&str, &str)| Op::Not {
            input: i.to_owned(),
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_assign(line: &str) -> IResult<&str, Op> {
    terminated(i32, tag(" -> "))
        .and(alpha1)
        .map(|(v, i): (i32, &str)| Op::Assign {
            input: i.to_owned(),
            value: v,
        })
        .parse(line)
}

fn parse_alias(line: &str) -> IResult<&str, Op> {
    terminated(alpha1, tag(" -> "))
        .and(alpha1)
        .map(|(i, o): (&str, &str)| Op::Alias {
            input: i.to_owned(),
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_and(line: &str) -> IResult<&str, Op> {
    let inputs = terminated(alphanumeric1, tag(" AND ")).and(alpha1);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i1, i2), o): ((&str, &str), &str)| Op::And {
            input1: i1.to_owned(),
            input2: i2.to_owned(),
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_or(line: &str) -> IResult<&str, Op> {
    let inputs = terminated(alpha1, tag(" OR ")).and(alpha1);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i1, i2), o): ((&str, &str), &str)| Op::Or {
            input1: i1.to_owned(),
            input2: i2.to_owned(),
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_lshift(line: &str) -> IResult<&str, Op> {
    let inputs = terminated(alpha1, tag(" LSHIFT ")).and(i32);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i, v), o): ((&str, i32), &str)| Op::LShift {
            input: i.to_owned(),
            value: v,
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_rshift(line: &str) -> IResult<&str, Op> {
    let inputs = terminated(alpha1, tag(" RSHIFT ")).and(i32);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i, v), o): ((&str, i32), &str)| Op::RShift {
            input: i.to_owned(),
            value: v,
            output: o.to_owned(),
        })
        .parse(line)
}

fn parse_operation(line: &str) -> IResult<&str, Op> {
    alt((
        parse_not,
        parse_alias,
        parse_assign,
        parse_and,
        parse_or,
        parse_lshift,
        parse_rshift,
    ))
    .parse(line)
}

fn parse_operations(lines: &str) -> Vec<Op> {
    lines
        .lines()
        .map(|line| parse_operation(line).unwrap().1)
        .collect()
}

struct Circuit<'a> {
    values: HashMap<&'a str, i32>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    fn lookup(&self, identifier: &str) -> Option<i32> {
        identifier
            .parse::<i32>()
            .ok()
            .or_else(|| self.get(identifier))
    }

    pub fn set(&mut self, signal: &'a str, value: i32) {
        self.values.insert(signal, value);
    }

    pub fn from(&mut self, operations: &'a [Op]) {
        let mut backlog = VecDeque::from_iter(operations);

        while let Some(op) = backlog.pop_front() {
            match op {
                Op::Not { input, output } => {
                    if let Some(v) = self.lookup(input) {
                        self.set(output, !v);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Assign { input, value } => {
                    self.set(input, *value);
                }
                Op::And { input1, input2, output } => {
                    if let (Some(v1), Some(v2)) = (self.lookup(input1), self.lookup(input2)) {
                        self.set(output, v1 & v2);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Or { input1, input2, output } => {
                    if let (Some(v1), Some(v2)) = (self.lookup(input1), self.lookup(input2)) {
                        self.set(output, v1 | v2);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::LShift { input, value, output } => {
                    if let Some(v) = self.lookup(input) {
                        self.values.insert(output, v << value);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::RShift { input, value, output } => {
                    if let Some(v) = self.lookup(input) {
                        self.set(output, v >> value);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Alias { input, output } => {
                    if let Some(v) = self.lookup(input) {
                        self.set(output, v);
                    } else {
                        backlog.push_back(op);
                    }
                }
            }
        }
    }

    pub fn get(&self, arg: &str) -> Option<i32> {
        self.values.get(arg).copied()
    }
}

pub struct Day7 {
    operations: Vec<Op>,
    operations_2: Vec<Op>,
}

impl Solution for Day7 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        Day7 {
            operations: parse_operations(input),
            operations_2: parse_operations(INPUT_2),
        }
    }

    fn fst(&self) -> i32 {
        let mut circuit = Circuit::new();

        circuit.from(&self.operations);

        circuit.get("a").unwrap()
    }

    fn snd(&self) -> i32 {
        let mut circuit = Circuit::new();

        circuit.from(&self.operations_2);

        circuit.get("a").unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day7::parse(INPUT);
        assert_eq!(day.fst(), 46065);
        assert_eq!(day.snd(), 14134);
    }
}
