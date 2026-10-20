use regex::Regex;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day3.txt");

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub struct Day3 {
    instructions: Vec<Instruction>,
}

impl Solution for Day3 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let re = Regex::new(r#"(mul\((?<op1>\d+),(?<op2>\d+)\))|(do\(\))|don't\(\)"#).unwrap();

        let mut instructions = vec![];
        for captures in re.captures_iter(input) {
            if captures.get(0).unwrap().as_str().starts_with("don't()") {
                instructions.push(Instruction::Dont);
                continue;
            }

            if captures.get(0).unwrap().as_str().starts_with("do()") {
                instructions.push(Instruction::Do);
                continue;
            }

            instructions.push(Instruction::Mul(
                captures["op1"].parse::<i32>().unwrap(),
                captures["op2"].parse::<i32>().unwrap(),
            ));
        }

        Day3 { instructions }
    }

    fn fst(&self) -> i32 {
        let mut sum = 0;
        for instruction in &self.instructions {
            if let Instruction::Mul(op1, op2) = instruction {
                sum += op1 * op2;
            }
        }

        sum
    }

    fn snd(&self) -> i32 {
        let mut sum = 0;
        let mut enabled = true;
        for instruction in &self.instructions {
            match instruction {
                Instruction::Dont => enabled = false,
                Instruction::Do => enabled = true,
                Instruction::Mul(op1, op2) => {
                    if enabled {
                        sum += op1 * op2;
                    }
                }
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
        let day = Day3::parse(INPUT);
        assert_eq!(day.fst(), 174561379);
        assert_eq!(day.snd(), 106921067);
    }
}
