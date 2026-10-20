use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::{many1, separated_list1},
};

#[derive(Clone)]
struct GameOfLife {
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    fn is_corner(&self, i: usize, j: usize) -> bool {
        (i == 0 || i == self.grid.len() - 1) && (j == 0 || j == self.grid[0].len() - 1)
    }

    fn count_neighbors(&self, i: usize, j: usize) -> usize {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        let row_start = i.saturating_sub(1);
        let row_end = (i + 1).min(rows - 1);
        let col_start = j.saturating_sub(1);
        let col_end = (j + 1).min(cols - 1);

        (row_start..=row_end)
            .flat_map(|r| (col_start..=col_end).map(move |c| (r, c)))
            .filter(|&(r, c)| (r, c) != (i, j) && self.grid[r][c])
            .count()
    }

    pub fn step(&mut self, corners_fixed: bool) {
        let mut new_grid = self.grid.clone();

        for (i, row) in new_grid.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if corners_fixed && self.is_corner(i, j) {
                    continue;
                }

                let count = self.count_neighbors(i, j);
                *cell = matches!((self.grid[i][j], count), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.grid = new_grid;
    }
}

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day18.txt");

pub struct Day18 {
    game: GameOfLife,
}

fn parse_input(content: &str) -> IResult<&str, GameOfLife> {
    let (content, grid) = separated_list1(tag("\n"), parse_line)(content)?;
    Ok((content, GameOfLife { grid }))
}

fn parse_line(content: &str) -> IResult<&str, Vec<bool>> {
    many1(alt((value(true, tag("#")), value(false, tag(".")))))(content)
}

impl Solution for Day18 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let game = parse_input(input)
            .expect("Should have been able to parse the input")
            .1;
        Day18 { game }
    }

    fn fst(&self) -> usize {
        let mut game = self.game.clone();
        for _ in 0..100 {
            game.step(false);
        }
        game.grid.iter().flatten().filter(|&&b| b).count()
    }

    fn snd(&self) -> usize {
        let mut game = self.game.clone();
        let rows = game.grid.len();
        let cols = game.grid[0].len();
        game.grid[0][0] = true;
        game.grid[0][cols - 1] = true;
        game.grid[rows - 1][0] = true;
        game.grid[rows - 1][cols - 1] = true;
        for _ in 0..100 {
            game.step(true);
        }
        game.grid.iter().flatten().filter(|&&b| b).count()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day18::parse(INPUT);
        assert_eq!(day.fst(), 1061);
        assert_eq!(day.snd(), 1006);
    }
}
