use std::fs;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::{many1, separated_list1},
};

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

const INPUT_FILE: &str = "src/aoc/y2015/day18.txt";

fn input() -> GameOfLife {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Should have been able to parse the input")
        .1
}

fn parse_input(content: &str) -> IResult<&str, GameOfLife> {
    let (content, grid) = separated_list1(tag("\n"), parse_line)(content)?;
    Ok((content, GameOfLife { grid }))
}

fn parse_line(content: &str) -> IResult<&str, Vec<bool>> {
    many1(alt((value(true, tag("#")), value(false, tag(".")))))(content)
}

pub fn day18_fst() -> usize {
    let mut game = input();
    for _ in 0..100 {
        game.step(false);
    }
    game.grid.iter().flatten().filter(|&&b| b).count()
}

pub fn day18_snd() -> usize {
    let mut game = input();
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_day18_fst() {
        assert_eq!(day18_fst(), 1061);
    }

    #[test]
    fn test_day18_snd() {
        assert_eq!(day18_snd(), 1006);
    }
}
