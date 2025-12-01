use std::fs;

use nom::{
    IResult,
    branch::alt,
    character::complete::{char, line_ending, u32},
    multi::separated_list1,
};

const INPUT_FILE: &str = "src/aoc/y2025/day1.txt";
const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn input() -> Vec<Rotations> {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    parse_content(&content)
        .expect("Failed to parse input content")
        .1
}

fn parse_content(content: &str) -> IResult<&str, Vec<Rotations>> {
    fn parse_rotation(input: &str) -> IResult<&str, Rotations> {
        let (input, direction) = alt((char('L'), char('R')))(input)?;
        let (input, value) = u32(input)?;

        let rotation = match direction {
            'L' => Rotations::L(value),
            'R' => Rotations::R(value),
            _ => unreachable!(),
        };

        Ok((input, rotation))
    }

    separated_list1(line_ending, parse_rotation)(content)
}

#[derive(Copy, Clone, Debug)]
enum Rotations {
    L(u32),
    R(u32),
}

#[derive(Debug)]
struct Safe {
    position: i32,
}

impl Safe {
    fn new() -> Self {
        Self { position: START_POS }
    }

    /// Rotate the safe's dial.
    /// Returns the number of times the dial passed zero.
    pub fn rotate(&mut self, rotation: &Rotations) -> u32 {
        let old_position = self.position;
        match rotation {
            Rotations::L(value) => {
                let full_rotations = value / 100;
                let remainder = (*value % 100) as i32;
                self.position = (self.position - remainder).rem_euclid(DIAL_SIZE);

                full_rotations
                    + if (self.position > old_position && old_position != 0) || self.position == 0 {
                        1
                    } else {
                        0
                    }
            }
            Rotations::R(value) => {
                let full_rotations = value / 100;
                let remainder = (*value % 100) as i32;
                self.position = (self.position + remainder).rem_euclid(DIAL_SIZE);
                full_rotations
                    + if (self.position < old_position && old_position != 0) || self.position == 0 {
                        1
                    } else {
                        0
                    }
            }
        }
    }
}

pub fn day1_fst() -> u32 {
    let rotations = input();
    let mut safe = Safe::new();

    let mut count = 0;
    for rotation in &rotations {
        safe.rotate(rotation);

        if safe.position == 0 {
            count += 1;
        }
    }

    count
}

pub fn day1_snd() -> u32 {
    let rotations = input();
    let mut safe = Safe::new();

    let mut count = 0;
    for rotation in &rotations {
        let passed_zero = safe.rotate(rotation);

        count += passed_zero;
    }

    count
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 999);
    }

    #[test]
    fn examples_snd() {
        let mut safe = Safe::new();

        assert_eq!(safe.rotate(&Rotations::L(68)), 1);
        assert_eq!(safe.rotate(&Rotations::L(30)), 0);
        assert_eq!(safe.rotate(&Rotations::R(48)), 1);
        assert_eq!(safe.rotate(&Rotations::L(5)), 0);
        assert_eq!(safe.rotate(&Rotations::R(60)), 1);
        assert_eq!(safe.rotate(&Rotations::L(55)), 1);
        assert_eq!(safe.rotate(&Rotations::L(1)), 0);
        assert_eq!(safe.rotate(&Rotations::L(99)), 1);
        assert_eq!(safe.rotate(&Rotations::R(14)), 0);
        assert_eq!(safe.rotate(&Rotations::L(82)), 1);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 6099);
    }
}
