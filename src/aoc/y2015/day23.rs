use nom::combinator::value;
use nom::character::complete::{i32, char};
use nom::bytes::complete::tag;
use nom::{multi::separated_list1, IResult};
use nom::branch::alt;

const INPUT_FILE: &str = "src/aoc/y2015/day23.txt";

fn input() -> Vec<Instructions> {
    let content = std::fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    parse_input(&content).expect("Could not parse input file").1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Instructions>> {
    separated_list1(char('\n'), parse_instruction)(content)
}

fn parse_instruction(line: &str) -> IResult<&str, Instructions> {
    let (rest, op_code) = alt((value(Op::Inc, tag("inc")),
                                value(Op::Tpl, tag("tpl")),
                                value(Op::Hlf, tag("hlf")),
                                value(Op::Jmp, tag("jmp")),
                                value(Op::Jie, tag("jie")),
                                value(Op::Jio, tag("jio"))))
                             (line)?;
    let (rest , _) = char(' ')(rest)?;
    if op_code == Op::Jmp {
        let (rest, offset) = i32(rest)?;
        return Ok((rest, Instructions { op: op_code, register: ' ', offet: offset }));
    }

    if op_code == Op::Jie || op_code == Op::Jio {
        let (rest, register) = alt((char('a'), char('b')))(rest)?;
        let (rest , _) = tag(", ")(rest)?;
        let (rest, offset) = i32(rest)?;
        return Ok((rest, Instructions { op: op_code, register, offet: offset }));
    }
    let (rest, register) = alt((char('a'), char('b')))(rest)?;
    Ok((rest, Instructions { op: op_code, register, offet: 0 }))
}


struct Instructions {
    op: Op,
    register: char,
    offet: i32,
}

#[derive(Clone, PartialEq)]
enum Op {
    Hlf,
    Tpl,
    Inc,
    Jmp,
    Jie,
    Jio,
}

struct CPU {
    registers: [u32; 2],
    pc: i32,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: [0; 2],
            pc: 0,
        }
    }
    fn setup(registers: [u32; 2]) -> Self {
        Self {
            registers,
            pc: 0,
        }
    }

    fn get_register_index(&self, reg: char) -> usize {
        match reg {
            'a' => 0,
            'b' => 1,
            _ => panic!("Invalid register"),
        }
    }

    fn step(&mut self, instruction: &Instructions) {
        match instruction.op {
            Op::Hlf => {
                let idx = self.get_register_index(instruction.register);
                self.registers[idx] /= 2;
                self.pc += 1;
            }
            Op::Tpl => {
                let idx = self.get_register_index(instruction.register);
                self.registers[idx] *= 3;
                self.pc += 1;
            }
            Op::Inc => {
                let idx = self.get_register_index(instruction.register);
                self.registers[idx] += 1;
                self.pc += 1;
            }
            Op::Jmp => {
                self.pc += instruction.offet;
            }
            Op::Jie => {
                let idx = self.get_register_index(instruction.register);
                if self.registers[idx] % 2 == 0 {
                    self.pc += instruction.offet;
                } else {
                    self.pc += 1;
                }
            }
            Op::Jio => {
                let idx = self.get_register_index(instruction.register);
                if self.registers[idx] == 1 {
                    self.pc += instruction.offet;
                } else {
                    self.pc += 1;
                }
            }
        }
    }

    fn run(&mut self, instructions: &[Instructions]) {
        while (self.pc as usize) < instructions.len() {
            let instruction = &instructions[self.pc as usize];
            self.step(instruction);
        }
    }
    
}


pub fn day23_fst() -> u32 {
    let instructions = input();
    let mut cpu = CPU::new();
    cpu.run(&instructions);
    cpu.registers[1]
}

pub fn day23_snd() -> u32 {
    let instructions = input();
    let mut cpu = CPU::setup([1, 0]);
    cpu.run(&instructions);
    cpu.registers[1]
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day23_fst(), 184);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day23_snd(), 231);
    }
}
