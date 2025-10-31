use std::collections::HashMap;

const INPUT: usize = 368078;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn neighboring_points(&self) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    neighbors.push(Point {
                        x: self.x + dx,
                        y: self.y + dy,
                    });
                }
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }
}

struct SpiralIterator {
    current: Point,
    direction: Direction,
    steps_in_direction: usize,
    steps_taken: usize,
    first: bool,
}

impl SpiralIterator {
    fn new() -> Self {
        Self {
            current: Point { x: 0, y: 0 },
            direction: Direction::Right,
            steps_in_direction: 1,
            steps_taken: 0,
            first: true,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.current);
        }

        // Move in current direction
        match self.direction {
            Direction::Right => self.current.x += 1,
            Direction::Up => self.current.y += 1,
            Direction::Left => self.current.x -= 1,
            Direction::Down => self.current.y -= 1,
        }

        self.steps_taken += 1;

        // Check if we need to turn
        if self.steps_taken == self.steps_in_direction {
            self.steps_taken = 0;
            self.direction = self.direction.next();

            // Increase step count after right and left directions
            if matches!(self.direction, Direction::Right | Direction::Left) {
                self.steps_in_direction += 1;
            }
        }

        Some(self.current)
    }
}

pub fn day3_fst() -> usize {
    SpiralIterator::new()
        .nth(INPUT - 1) // 0-based index
        .map(|point| point.manhattan_distance())
        .unwrap_or(0)
}

/// Solution can be looked up from OEIS: https://oeis.org/A141481
pub fn day3_snd() -> usize {
    SpiralIterator::new()
        .skip(1) // first point is inserted in initial state of scan
        .scan(
            HashMap::from([(Point { x: 0, y: 0 }, 1)]),
            |state, point| {
                let value = point
                    .neighboring_points()
                    .iter()
                    .map(|p| *state.get(p).unwrap_or(&0))
                    .sum();
                state.insert(point, value);
                Some(value)
            },
        )
        .find(|&value| value > INPUT)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral_examples() {
        let first_ten = SpiralIterator::new().take(10).collect::<Vec<_>>();
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -1 },
        ];

        assert_eq!(first_ten, expected);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day3_fst(), 371);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day3_snd(), 369601);
    }
}
