use itertools::{Either, Itertools};
use std::collections::HashSet;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day3.txt");

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn walk(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Left => Position::new(self.x - 1, self.y),
        }
    }
}

fn positions(directions: Vec<Direction>) -> Vec<Position> {
    let mut pos = Position::origin();
    std::iter::once(pos)
        .chain(directions.into_iter().map(move |dir| {
            pos = pos.walk(dir);
            pos
        }))
        .collect()
}

fn split(directions: Vec<Direction>) -> (Vec<Direction>, Vec<Direction>) {
    directions.into_iter().enumerate().partition_map(|(i, d)| {
        if i % 2 == 0 {
            Either::Left(d)
        } else {
            Either::Right(d)
        }
    })
}

fn split_positions(directions: Vec<Direction>) -> (Vec<Position>, Vec<Position>) {
    let (directions_santa, directions_robot) = split(directions);
    (positions(directions_santa), positions(directions_robot))
}

pub struct Day3 {
    directions: Vec<Direction>,
}

impl Solution for Day3 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let directions = input
            .chars()
            .map(|m| match m {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => panic!("Invalid input"),
            })
            .collect();
        Day3 { directions }
    }

    fn fst(&self) -> usize {
        positions(self.directions.clone())
            .into_iter()
            .collect::<HashSet<Position>>()
            .len()
    }

    fn snd(&self) -> usize {
        let (positions_santa, positions_robot) = split_positions(self.directions.clone());

        positions_santa
            .into_iter()
            .chain(positions_robot)
            .collect::<HashSet<Position>>()
            .len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day3::parse(INPUT);
        assert_eq!(day.fst(), 2565);
        assert_eq!(day.snd(), 2639);
    }
}
