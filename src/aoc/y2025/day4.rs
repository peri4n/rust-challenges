use std::fs;

const INPUT_FILE: &str = "src/aoc/y2025/day4.txt";

fn input() -> Vec<Vec<char>> {
    fs::read_to_string(INPUT_FILE)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn day4_fst() -> usize {
    let grid_data = input();

    let grid = Grid::new(grid_data);
    grid.count_accessible()
}

pub fn day4_snd() -> u32 {
    let grid_data = input();

    let mut grid = Grid::new(grid_data);
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
    fn solution_fst() {
        assert_eq!(day4_fst(), 1564);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day4_snd(), 9401);
    }
}

