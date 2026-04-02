use itertools::{Either, Itertools};
use std::{collections::HashSet, fs};

const INPUT_FILE: &str = "src/aoc/y2015/day3.txt";

#[derive(Debug)]
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

fn directions() -> Vec<Direction> {
    let line = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    line.chars()
        .map(|m| match m {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Invalid input"),
        })
        .collect()
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

pub fn day3_fst() -> usize {
    let dir = directions();
    positions(dir)
        .into_iter()
        .collect::<HashSet<Position>>()
        .len()
}

pub fn day3_snd() -> usize {
    let dir = directions();
    let (positions_santa, positions_robot) = split_positions(dir);

    positions_santa
        .into_iter()
        .chain(positions_robot)
        .collect::<HashSet<Position>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day3_fst(), 2565);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day3_snd(), 2639);
    }
}
