use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u32};
use nom::combinator::value;
use nom::sequence::{separated_pair, terminated};
use std::fs;

use nom::{IResult, Parser};

type Lights = Vec<Vec<u8>>;

const INPUT_FILE: &str = "src/aoc/y2015/day6.txt";

#[derive(Clone, Debug)]
enum Kind {
    Off,
    On,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    top_left: (u32, u32),
    bottom_right: (u32, u32),
    kind: Kind,
}

impl Instruction {
    pub fn exec(&self, lights: &mut Lights) {
        match self.kind {
            Kind::Off => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        lights[i as usize][j as usize] = 0;
                    }
                }
            }
            Kind::On => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        lights[i as usize][j as usize] = 1;
                    }
                }
            }
            Kind::Toggle => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        lights[i as usize][j as usize] = 1 - lights[i as usize][j as usize]
                    }
                }
            }
        }
    }

    pub fn exec2(&self, lights: &mut Lights) {
        match self.kind {
            Kind::Off => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        let brightness = &mut lights[i as usize][j as usize];

                        if *brightness > 0 {
                            *brightness -= 1;
                        }
                    }
                }
            }
            Kind::On => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        lights[i as usize][j as usize] += 1;
                    }
                }
            }
            Kind::Toggle => {
                for i in self.top_left.0..=self.bottom_right.0 {
                    for j in self.top_left.1..=self.bottom_right.1 {
                        lights[i as usize][j as usize] += 2;
                    }
                }
            }
        }
    }
}

fn input() -> Vec<Instruction> {
    fs::read_to_string(INPUT_FILE)
        .expect("Unable to read the input file")
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_kind(kind: &str) -> IResult<&str, Kind> {
    alt((
        value(Kind::Off, tag("turn off")),
        value(Kind::On, tag("turn on")),
        value(Kind::Toggle, tag("toggle")),
    ))(kind)
}

fn parse_coordinate(coord: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag(","), u32)(coord)
}

fn parse_line(line: &str) -> IResult<&str, Instruction> {
    let (rest, ((kind, top_left), bottom_right)) = terminated(parse_kind, char(' '))
        .and(terminated(parse_coordinate, tag(" through ")))
        .and(parse_coordinate)
        .parse(line)?;

    Ok((
        rest,
        Instruction {
            top_left,
            bottom_right,
            kind,
        },
    ))
}

pub fn day6_fst() -> i32 {
    let mut lights = vec![vec![0; 1000]; 1000];
    for instruction in input() {
        instruction.exec(&mut lights);
    }

    let mut lights_switched_on = 0;
    for light in lights {
        for b in light {
            if b == 1 {
                lights_switched_on += 1;
            }
        }
    }

    lights_switched_on
}

pub fn day6_snd() -> i32 {
    let mut lights = vec![vec![0; 1000]; 1000];
    for instruction in input() {
        instruction.exec2(&mut lights);
    }

    let mut brightnesses = 0_i32;
    for light in lights {
        for b in light {
            brightnesses += b as i32;
        }
    }

    brightnesses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day6_fst(), 377891);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day6_snd(), 14110788);
    }
}
