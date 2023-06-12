use std::{collections::HashSet, fs};

const INPUT_FILE: &str = "src/aoc/y2015/day3.txt";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn of(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Position {
    pub fn walk(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position::of(self.x, self.y + 1),
            Direction::Down => Position::of(self.x, self.y - 1),
            Direction::Right => Position::of(self.x + 1, self.y),
            Direction::Left => Position::of(self.x - 1, self.y),
        }
    }
}

fn directions() -> Vec<Direction> {
    let line = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut res = Vec::with_capacity(line.len());
    for m in line.chars() {
        let direction = match m {
            '>' => Direction::Left,
            '<' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Invalid input"),
        };

        res.push(direction);
    }

    res
}

fn positions(directions: Vec<Direction>) -> Vec<Position> {
    let mut visited = Vec::with_capacity(directions.len());

    let mut position = Position::origin();

    for dir in directions {
        let new_position = position.walk(dir);
        visited.push(position);
        position = new_position;
    }
    visited.push(position);

    visited
}
fn split(directions: Vec<Direction>) -> (Vec<Direction>, Vec<Direction>) {
    let mut first = Vec::with_capacity(directions.len() / 2);
    let mut second = Vec::with_capacity(directions.len() / 2);

    let mut token = true;
    for dir in directions {
        if token {
            first.push(dir);
        } else {
            second.push(dir);
        }
        token = !token;
    }

    (first, second)
}

fn split_positions(directions: Vec<Direction>) -> (Vec<Position>, Vec<Position>) {
    let (directions_santa, directions_robot) = split(directions);
    (positions(directions_santa), positions(directions_robot))
}

pub fn day3_fst() -> usize {
    let dir = directions();
    positions(dir)
        .into_iter()
        .collect::<HashSet<Position>>().len()
}

pub fn day3_snd() -> usize {
    let dir = directions();
    let (mut positions_santa, mut positions_robot) = split_positions(dir);
    positions_santa.append(&mut positions_robot);

    positions_santa
        .into_iter()
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
