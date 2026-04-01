use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u32 as nom_u32};
use nom::combinator::{map, value};
use nom::sequence::{separated_pair, terminated};
use std::fs;

use nom::{IResult, Parser};

type Lights = Vec<Vec<u32>>;

const INPUT_FILE: &str = "src/aoc/y2015/day6.txt";

#[derive(Clone, Debug)]
enum Kind {
    Off,
    On,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    kind: Kind,
}

impl Instruction {
    fn for_each_cell(&self, lights: &mut Lights, mut f: impl FnMut(&mut u32)) {
        for row in &mut lights[self.top_left.0..=self.bottom_right.0] {
            for cell in &mut row[self.top_left.1..=self.bottom_right.1] {
                f(cell);
            }
        }
    }

    pub fn exec(&self, lights: &mut Lights) {
        self.for_each_cell(lights, |cell| match self.kind {
            Kind::Off => *cell = 0,
            Kind::On => *cell = 1,
            Kind::Toggle => *cell = 1 - *cell,
        });
    }

    pub fn exec2(&self, lights: &mut Lights) {
        self.for_each_cell(lights, |cell| match self.kind {
            Kind::Off => *cell = cell.saturating_sub(1),
            Kind::On => *cell += 1,
            Kind::Toggle => *cell += 2,
        });
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

fn parse_coordinate(coord: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map(nom_u32, |v| v as usize),
        tag(","),
        map(nom_u32, |v| v as usize),
    )(coord)
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

pub fn day6_fst() -> usize {
    input()
        .into_iter()
        .fold(vec![vec![0; 1000]; 1000], |mut lights, instruction| {
            instruction.exec(&mut lights);
            lights
        })
        .iter()
        .flatten()
        .filter(|&b| *b == 1)
        .count()
}

pub fn day6_snd() -> u32 {
    input()
        .into_iter()
        .fold(vec![vec![0; 1000]; 1000], |mut lights, instruction| {
            instruction.exec2(&mut lights);
            lights
        })
        .iter()
        .flatten()
        .sum()
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
