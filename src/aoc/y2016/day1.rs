use std::{collections::HashSet, fs};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u32},
    error::Error,
    multi::separated_list0,
    IResult,
};

const INPUT_FILE: &str = "src/aoc/y2016/day1.txt";

fn input() -> Vec<Directive> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Something went wrong while parsing")
        .1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Directive>> {
    separated_list0(tag(", "), parse_directive)(&content)
}

fn parse_directive(content: &str) -> IResult<&str, Directive> {
    let (content, direction) = alt((char('L'), char('R')))(content)?;
    let (content, count) = u32(content)?;

    return match direction {
        'R' => Ok((content, Directive::Right(count))),
        'L' => Ok((content, Directive::Left(count))),
        _ => Err(nom::Err::Error(Error::new(
            "Unexpected direction",
            nom::error::ErrorKind::Char,
        ))),
    };
}

#[derive(Debug)]
enum Directive {
    Left(u32),
    Right(u32),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn to(&self, other: &Self) -> Vec<Self> {
        if self.x == other.x {
            return (self.y..other.y)
                .map(|y| Position::new(self.x, y))
                .collect_vec();
        }

        if self.y == other.y {
            return (self.x..other.x)
                .map(|x| Position::new(x, self.y))
                .collect_vec();
        }

        panic!("Should never happen!")
    }
    pub fn distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

#[derive(Debug)]
struct Walker {
    direction: Direction,
    pub position: Position,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            direction: Direction::North,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn walk(&mut self, directive: &Directive) {
        match (&self.direction, directive) {
            (Direction::North, Directive::Left(l)) => {
                self.direction = Direction::West;
                self.position.x -= *l as i32;
            }
            (Direction::North, Directive::Right(r)) => {
                self.direction = Direction::East;
                self.position.x += *r as i32;
            }
            (Direction::East, Directive::Left(l)) => {
                self.direction = Direction::North;
                self.position.y += *l as i32;
            }
            (Direction::East, Directive::Right(r)) => {
                self.direction = Direction::South;
                self.position.y -= *r as i32;
            }
            (Direction::South, Directive::Left(l)) => {
                self.direction = Direction::East;
                self.position.x += *l as i32;
            }
            (Direction::South, Directive::Right(r)) => {
                self.direction = Direction::West;
                self.position.x -= *r as i32;
            }
            (Direction::West, Directive::Left(l)) => {
                self.direction = Direction::South;
                self.position.y -= *l as i32;
            }
            (Direction::West, Directive::Right(r)) => {
                self.direction = Direction::North;
                self.position.y += *r as i32;
            }
        }
    }

    pub fn distance(&self) -> u32 {
        self.position.distance()
    }
}

pub fn day1_fst() -> u32 {
    let directives = input();
    let mut state = Walker::new();

    for directive in directives {
        state.walk(&directive);
    }

    state.distance()
}

pub fn day1_snd() -> u32 {
    let directives = input();
    let mut state = Walker::new();
    let mut visited = HashSet::new();

    for directive in directives {
        let old_position = state.position;
        state.walk(&directive);

        for pos in old_position.to(&state.position) {
            if visited.contains(&pos) {
                return pos.x.abs() as u32 + pos.y.abs() as u32;
            }
            visited.insert(pos);
        }
    }

    state.distance()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_fst1() {
        let mut state = Walker::new();
        state.walk(&Directive::Right(2));
        state.walk(&Directive::Left(3));

        assert_eq!(state.distance(), 5);
    }

    #[test]
    fn examples_fst2() {
        let mut state = Walker::new();
        state.walk(&Directive::Right(2));
        state.walk(&Directive::Right(2));
        state.walk(&Directive::Right(2));

        assert_eq!(state.distance(), 2);
    }

    #[test]
    fn examples_fst3() {
        let mut state = Walker::new();
        state.walk(&Directive::Right(5));
        state.walk(&Directive::Left(5));
        state.walk(&Directive::Right(5));
        state.walk(&Directive::Right(3));

        assert_eq!(state.distance(), 12);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 271);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 153);
    }
}
