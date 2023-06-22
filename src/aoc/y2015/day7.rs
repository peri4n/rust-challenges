use std::collections::{HashMap, VecDeque};
use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, i32};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

const INPUT_FILE: &str = "src/aoc/y2015/day7.txt";

const INPUT_FILE_2: &str = "src/aoc/y2015/day7_2.txt";

#[derive(Debug)]
enum Op<'a> {
    Not {
        input: &'a str,
        output: &'a str,
    },
    Assign {
        input: &'a str,
        value: i32,
    },
    And {
        input1: &'a str,
        input2: &'a str,
        output: &'a str,
    },
    Or {
        input1: &'a str,
        input2: &'a str,
        output: &'a str,
    },
    LShift {
        input: &'a str,
        value: i32,
        output: &'a str,
    },
    RShift {
        input: &'a str,
        value: i32,
        output: &'a str,
    },
    Alias {
        input: &'a str,
        output: &'a str,
    },
}

fn parse_not<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    let input = preceded(tag("NOT "), alpha1);
    let output = preceded(tag(" -> "), alpha1);
    input
        .and(output)
        .map(|(i, o)| Op::Not {
            input: i,
            output: o,
        })
        .parse(line)
}

fn parse_assign<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    terminated(i32, tag(" -> "))
        .and(alpha1)
        .map(|(v, i)| Op::Assign { input: i, value: v })
        .parse(line)
}

fn parse_alias<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    terminated(alpha1, tag(" -> "))
        .and(alpha1)
        .map(|(i, o)| Op::Alias {
            input: i,
            output: o,
        })
        .parse(line)
}

fn parse_and<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    let inputs = terminated(alphanumeric1, tag(" AND ")).and(alpha1);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i1, i2), o)| Op::And {
            input1: i1,
            input2: i2,
            output: o,
        })
        .parse(line)
}

fn parse_or<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    let inputs = terminated(alpha1, tag(" OR ")).and(alpha1);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i1, i2), o)| Op::Or {
            input1: i1,
            input2: i2,
            output: o,
        })
        .parse(line)
}

fn parse_lshift<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    let inputs = terminated(alpha1, tag(" LSHIFT ")).and(i32);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i, v), o)| Op::LShift {
            input: i,
            value: v,
            output: o,
        })
        .parse(line)
}

fn parse_rshift<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
    let inputs = terminated(alpha1, tag(" RSHIFT ")).and(i32);
    let output = preceded(tag(" -> "), alpha1);
    inputs
        .and(output)
        .map(|((i, v), o)| Op::RShift {
            input: i,
            value: v,
            output: o,
        })
        .parse(line)
}

fn parse_operation<'a>(line: &'a str) -> IResult<&str, Op<'a>> {
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

fn read_definition(file: &str) -> String {
    fs::read_to_string(file).expect("Unable to read input file")
}

fn parse_operations(lines: &str) -> Vec<Op> {
    lines
        .lines()
        .map(|line| parse_operation(&line).unwrap().1)
        .collect()
}

struct Circuit<'a> {
    values: HashMap<&'a str, i32>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn lookup(&self, identifier: &str) -> Option<i32> {
        identifier
            .parse::<i32>()
            .ok()
            .or_else(|| self.get(identifier))
    }

    pub fn set(&mut self, signal: &'a str, value:i32) {
        self.values.insert(signal, value);
    }

    pub fn from(&mut self, definition: &'a str) {
        let operations = parse_operations(definition);

        let mut backlog = VecDeque::from_iter(operations);

        while let Some(op) = backlog.pop_front() {
            match op {
                Op::Not { input, output } => {
                    if let Some(v) = self.lookup(&input) {
                        self.set(output, !v);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Assign { input, value } => {
                    self.set(input, value);
                }
                Op::And {
                    input1,
                    input2,
                    output,
                } => {
                    if let (Some(v1), Some(v2)) =
                        (self.lookup(&input1), self.lookup(&input2))
                    {
                        self.set(output, v1 & v2);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Or {
                    input1,
                    input2,
                    output,
                } => {
                    if let (Some(v1), Some(v2)) =
                        (self.lookup(&input1), self.lookup(&input2))
                    {
                        self.set(output, v1 | v2);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::LShift {
                    input,
                    value,
                    output,
                } => {
                    if let Some(v) = self.lookup(&input) {
                        self.values.insert(output, v << value);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::RShift {
                    input,
                    value,
                    output,
                } => {
                    if let Some(v) = self.lookup(&input) {
                        self.set(output, v >> value);
                    } else {
                        backlog.push_back(op);
                    }
                }
                Op::Alias { input, output } => {
                    if let Some(v) = self.lookup(&input) {
                        self.set(output, v);
                    } else {
                        backlog.push_back(op);
                    }
                }
            }
        }
    }

    pub fn get(&self, arg: &str) -> Option<i32> {
        self.values.get(arg).map(|e| *e)
    }
}

fn day7_fst() -> i32 {
    let mut circuit = Circuit::new();

    let definition = read_definition(INPUT_FILE);
    circuit.from(&definition);

    circuit.get("a").unwrap()
}

fn day7_snd() -> i32 {
    let mut circuit = Circuit::new();

    let definition = read_definition(INPUT_FILE_2);

    circuit.from(&definition);

    circuit.get("a").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day7_fst(), 46065);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day7_snd(), 14134);
    }
}
