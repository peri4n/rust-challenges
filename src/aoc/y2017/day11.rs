use nom::{IResult, branch::alt, bytes::complete::tag, combinator::value, multi::separated_list1};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day11.txt");

#[derive(Clone, Debug)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

fn parse_input(content: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(tag(","), parse_direction)(content)
}

fn parse_direction(content: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::SouthEast, tag("se")),
        value(Direction::SouthWest, tag("sw")),
        value(Direction::NorthWest, tag("nw")),
        value(Direction::NorthEast, tag("ne")),
        value(Direction::South, tag("s")),
        value(Direction::North, tag("n")),
    ))(content)
}

struct HexTile {
    x: i32,
    y: i32,
    z: i32,
}

impl HexTile {
    pub fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                self.y += 1;
                self.z -= 1;
            }
            Direction::NorthEast => {
                self.x += 1;
                self.z -= 1;
            }
            Direction::SouthEast => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::South => {
                self.y -= 1;
                self.z += 1;
            }
            Direction::SouthWest => {
                self.x -= 1;
                self.z += 1;
            }
            Direction::NorthWest => {
                self.x -= 1;
                self.y += 1;
            }
        }
    }

    pub fn distance_to_origin(&self) -> usize {
        self.x.abs().max(self.z.abs()).max(self.y.abs()) as usize
    }
}

pub struct Day11 {
    directions: Vec<Direction>,
}

impl Solution for Day11 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let directions = parse_input(input)
            .expect("Something went wrong while parsing")
            .1;
        Day11 { directions }
    }

    fn fst(&self) -> usize {
        let mut tile = HexTile::origin();
        for direction in self.directions.iter().cloned() {
            tile.walk(direction)
        }
        tile.distance_to_origin()
    }

    fn snd(&self) -> usize {
        let mut tile = HexTile::origin();
        self.directions.iter().cloned().fold(0, |acc, direction| {
            tile.walk(direction);
            acc.max(tile.distance_to_origin())
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day11::parse(INPUT);
        assert_eq!(day.fst(), 805);
        assert_eq!(day.snd(), 1535);
    }
}
