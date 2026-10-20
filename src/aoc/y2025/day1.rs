use nom::{
    IResult,
    branch::alt,
    character::complete::{char, line_ending, u32},
    multi::separated_list1,
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");
const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

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

pub struct Day1 {
    rotations: Vec<Rotations>,
}

impl Solution for Day1 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let rotations = parse_content(input)
            .expect("Failed to parse input content")
            .1;
        Day1 { rotations }
    }

    fn fst(&self) -> u32 {
        let mut safe = Safe::new();

        let mut count = 0;
        for rotation in &self.rotations {
            safe.rotate(rotation);

            if safe.position == 0 {
                count += 1;
            }
        }

        count
    }

    fn snd(&self) -> u32 {
        let mut safe = Safe::new();

        let mut count = 0;
        for rotation in &self.rotations {
            let passed_zero = safe.rotate(rotation);

            count += passed_zero;
        }

        count
    }
}

#[cfg(test)]
mod test {

    use super::*;

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
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 999);
        assert_eq!(day.snd(), 6099);
    }
}
