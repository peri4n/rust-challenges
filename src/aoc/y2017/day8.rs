use std::{collections::HashMap, fs};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, newline},
    combinator::{complete, value},
    multi::separated_list1,
};

const INPUT_FILE: &str = "src/aoc/y2017/day8.txt";

pub fn day8_fst() -> i32 {
    let input = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let (_, instructions) = parse_content(&input).expect("Could not parse input file");
    let registers = instructions
        .iter()
        .map(|inst| inst.register.0.as_ref())
        .collect();

    let mut cpu = CPU::new(registers);
    cpu.execute_all(instructions.into_iter());

    *cpu.max_register().unwrap_or(&0)
}

pub fn day8_snd() -> i32 {
    let input = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let (_, instructions) = parse_content(&input).expect("Could not parse input file");
    let registers = instructions
        .iter()
        .map(|inst| inst.register.0.as_ref())
        .collect();

    let mut cpu = CPU::new(registers);
    let mut total_max = i32::MIN;
    for inst in instructions {
        cpu.execute(&inst);

        let current_max = cpu.max_register().unwrap_or(&i32::MIN);
        if *current_max > total_max {
            total_max = *current_max;
        }
    }

    total_max
}
struct CPU {
    registers: HashMap<Register, i32>,
}

impl CPU {
    fn new(registers: Vec<&str>) -> Self {
        Self {
            registers: registers
                .iter()
                .map(|r| (Register(r.to_string()), 0))
                .collect(),
        }
    }

    fn max_register(&self) -> Option<&i32> {
        self.registers.values().max()
    }
}

impl CPU {
    fn execute_all<T>(&mut self, instructions: T)
    where
        T: Iterator<Item = Instruction>,
    {
        for inst in instructions {
            self.execute(&inst);
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        if self.evaluate(&instruction.condition) {
            match instruction.operation {
                Op::Dec => {
                    *self.registers.get_mut(&instruction.register).unwrap() -= instruction.value
                }
                Op::Inc => {
                    *self.registers.get_mut(&instruction.register).unwrap() += instruction.value
                }
            }
        }
    }

    fn evaluate(&self, cond: &Condition) -> bool {
        self.registers
            .get(&cond.register)
            .map(|r| match cond.comparison {
                Comparison::LT => r < &cond.constant,
                Comparison::LTE => r <= &cond.constant,
                Comparison::EQ => r == &cond.constant,
                Comparison::NEQ => r != &cond.constant,
                Comparison::GT => r > &cond.constant,
                Comparison::GTE => r >= &cond.constant,
            })
            .unwrap_or(false)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Register(String);

#[derive(Clone, Debug)]
enum Op {
    Inc,
    Dec,
}

#[derive(Clone, Debug)]
enum Comparison {
    LT,
    LTE,
    EQ,
    NEQ,
    GT,
    GTE,
}

#[derive(Debug)]
struct Condition {
    register: Register,
    comparison: Comparison,
    constant: i32,
}

#[derive(Debug)]
struct Instruction {
    register: Register,
    operation: Op,
    value: i32,
    condition: Condition,
}

fn parse_content(content: &str) -> IResult<&str, Vec<Instruction>> {
    complete(separated_list1(newline, parse_line))(content)
}

fn parse_line(line: &str) -> IResult<&str, Instruction> {
    let (rest, reg) = alpha1(line)?;
    let (rest, op) = alt((value(Op::Inc, tag(" inc ")), value(Op::Dec, tag(" dec "))))(rest)?;
    let (rest, op_value) = i32(rest)?;
    let (rest, _) = tag(" if ")(rest)?;
    let (rest, cond_reg) = alpha1(rest)?;
    let (rest, comparison) = alt((
        value(Comparison::LT, tag(" < ")),
        value(Comparison::LTE, tag(" <= ")),
        value(Comparison::EQ, tag(" == ")),
        value(Comparison::NEQ, tag(" != ")),
        value(Comparison::GTE, tag(" >= ")),
        value(Comparison::GT, tag(" > ")),
    ))(rest)?;
    let (rest, cond_value) = i32(rest)?;

    Ok((
        rest,
        Instruction {
            register: Register(reg.to_string()),
            operation: op,
            value: op_value,
            condition: Condition {
                register: Register(cond_reg.to_string()),
                comparison: comparison,
                constant: cond_value,
            },
        },
    ))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day8_fst(), 6828);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day8_snd(), 7234);
    }
}
