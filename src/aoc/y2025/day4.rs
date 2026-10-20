use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day4.txt");

pub struct Day4 {
    grid: Grid,
}

impl Solution for Day4 {
    type Fst = usize;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let grid_data = input.lines().map(|line| line.chars().collect()).collect();
        Day4 {
            grid: Grid::new(grid_data),
        }
    }

    fn fst(&self) -> usize {
        self.grid.count_accessible()
    }

    fn snd(&self) -> u32 {
        let mut grid = self.grid.clone();
        let mut total_removed = 0;

        loop {
            let removed = grid.remove_all_accessible();
            if removed == 0 {
                break;
            }
            total_removed += removed;
        }

        total_removed
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<char>>) -> Self {
        let rows = cells.len();
        let cols = if rows > 0 { cells[0].len() } else { 0 };
        Grid { cells, rows, cols }
    }

    fn neighbours_of(&self, r: usize, c: usize) -> Vec<char> {
        let mut neighbours = Vec::new();
        let directions = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                neighbours.push(self.cells[nr as usize][nc as usize]);
            }
        }
        neighbours
    }

    fn is_accessible(&self, r: usize, c: usize) -> bool {
        if self.cells[r][c] != '@' {
            return false;
        }
        let mut occupied = 0;
        let neighbours = self.neighbours_of(r, c);
        for &n in neighbours.iter() {
            if n == '@' {
                occupied += 1;
            }
        }
        occupied < 4
    }

    fn remove_all_accessible(&mut self) -> u32 {
        let mut changed = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_accessible(r, c) {
                    self.cells[r][c] = '.';
                    changed += 1;
                }
            }
        }
        changed
    }

    fn count_accessible(&self) -> usize {
        let mut count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_accessible(r, c) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day4::parse(INPUT);
        assert_eq!(day.fst(), 1564);
        assert_eq!(day.snd(), 9401);
    }
}
