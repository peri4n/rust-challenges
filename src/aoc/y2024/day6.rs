use std::collections::HashSet;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day6.txt");

pub struct Day6 {
    grid: Grid,
}

impl Solution for Day6 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        Day6 {
            grid: Grid::from_string(input),
        }
    }

    fn fst(&self) -> i32 {
        let mut grid = self.grid.clone();
        let mut positions = HashSet::new();

        while grid.step() {
            positions.insert(grid.current_position);
        }

        positions.len() as i32
    }

    fn snd(&self) -> i32 {
        let mut grid = self.grid.clone();
        let start_pos = grid.current_position;
        let start_dir = grid.direction;
        let mut positions = HashSet::new();
        let mut nr_loops = 0;

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                if !grid.position_blocked((x, y)) {
                    grid.set_start_position_direction(start_pos, start_dir);

                    grid.block((x, y));
                    // detect if we can run in a loop
                    let mut loop_detected = false;

                    while grid.step() && !loop_detected {
                        if positions.contains(&(grid.current_position, grid.direction)) {
                            nr_loops += 1;
                            loop_detected = true;
                        }
                        positions.insert((grid.current_position, grid.direction));
                    }

                    positions.clear();
                    grid.unblock((x, y));
                }
            }
        }

        nr_loops
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    data: Vec<Vec<char>>,
    current_position: (usize, usize),
    direction: Direction,
}

impl Grid {
    fn from_string(content: &str) -> Self {
        let mut data = Vec::new();
        let (mut x, mut y) = (0, 0);
        let mut direction = Direction::Up;

        for (i, line) in content.lines().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            for (j, c) in chars.iter().enumerate() {
                match c {
                    '>' => {
                        direction = Direction::Right;
                        x = j;
                        y = i;
                    }
                    '<' => {
                        direction = Direction::Left;
                        x = j;
                        y = i;
                    }
                    '^' => {
                        direction = Direction::Up;
                        x = j;
                        y = i;
                    }
                    'v' => {
                        direction = Direction::Down;
                        x = j;
                        y = i;
                    }
                    _ => continue,
                }
            }
            if chars.contains(&'>') {
                y = i;
                x = chars.iter().position(|&c| c == '>').unwrap();
            }
            data.push(chars);
        }

        Grid {
            data,
            direction,
            current_position: (x, y),
        }
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn set_start_position_direction(&mut self, position: (usize, usize), direction: Direction) {
        self.current_position = position;
        self.direction = direction;
    }

    /// Returns the next position based on the current direction.
    /// If the next position is out of bounds, it returns None.
    fn next_position(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => {
                if self.current_position.1 > 0 {
                    Some((self.current_position.0, self.current_position.1 - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.current_position.1 < self.data.len() - 1 {
                    Some((self.current_position.0, self.current_position.1 + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.current_position.0 > 0 {
                    Some((self.current_position.0 - 1, self.current_position.1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.current_position.0 < self.data[0].len() - 1 {
                    Some((self.current_position.0 + 1, self.current_position.1))
                } else {
                    None
                }
            }
        }
    }

    fn block(&mut self, position: (usize, usize)) {
        self.data[position.1][position.0] = '#';
    }

    fn unblock(&mut self, position: (usize, usize)) {
        self.data[position.1][position.0] = '.';
    }

    fn position_blocked(&self, position: (usize, usize)) -> bool {
        self.data[position.1][position.0] == '#'
    }

    /// Returns false if the step has left the grid
    pub fn step(&mut self) -> bool {
        if let Some(next_pos) = self.next_position() {
            if self.position_blocked(next_pos) {
                self.direction = self.direction.turn_right();
                return self.step();
            }
            self.current_position = next_pos;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        let grid = Grid::from_string(
            "....#.....\n\
                .........#\n\
                ..........\n\
                ..#.......\n\
                .......#..\n\
                ..........\n\
                .#..^.....\n\
                ........#.\n\
                #.........\n\
                ......#...",
        );

        assert_eq!(grid.current_position, (4, 6));
        assert_eq!(grid.direction, Direction::Up);
    }

    #[test]
    fn moves_up() {
        let mut grid = Grid::from_string(
            "....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...",
        );

        grid.step();
        assert_eq!(grid.current_position, (4, 5));
        assert_eq!(grid.direction, Direction::Up);
    }

    #[test]
    fn moves_up_blocked() {
        let mut grid = Grid::from_string(
            "....#.....\n\
                ....^....#\n\
                ..........\n\
                ..#.......\n\
                .......#..\n\
                ..........\n\
                .#........\n\
                ........#.\n\
                #.........\n\
                ......#...",
        );

        grid.step();
        assert_eq!(grid.current_position, (5, 1));
        assert_eq!(grid.direction, Direction::Right);
    }

    #[test]
    #[ignore = "To slow"]
    fn solution() {
        let day = Day6::parse(INPUT);
        assert_eq!(day.fst(), 5067);
        assert_eq!(day.snd(), 1793);
    }
}
