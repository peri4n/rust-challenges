use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

struct GameOfLife {
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn next(&mut self) {
        let mut new_grid = self.grid.clone();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let mut count = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let new_i = i as i32 + x;
                        let new_j = j as i32 + y;
                        if new_i >= 0
                            && new_i < self.grid.len() as i32
                            && new_j >= 0
                            && new_j < self.grid[i].len() as i32
                            && self.grid[new_i as usize][new_j as usize]
                        {
                            count += 1;
                        }
                    }
                }

                new_grid[i][j] =
                    matches!((self.grid[i][j], count), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.grid = new_grid;
    }

    pub fn next2(&mut self) {
        let mut new_grid = self.grid.clone();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if (i == 0 || i == self.grid.len() - 1) && (j == 0 || j == self.grid[i].len() - 1) {
                    continue;
                }

                let mut count = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let new_i = i as i32 + x;
                        let new_j = j as i32 + y;
                        if new_i >= 0
                            && new_i < self.grid.len() as i32
                            && new_j >= 0
                            && new_j < self.grid[i].len() as i32
                            && self.grid[new_i as usize][new_j as usize]
                        {
                            count += 1;
                        }
                    }
                }

                new_grid[i][j] =
                    matches!((self.grid[i][j], count), (true, 2) | (true, 3) | (false, 3));
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

pub fn day18_fst() -> i32 {
    let mut game = input();
    for _ in 0..100 {
        game.next();
    }
    game.grid.iter().flatten().filter(|&&b| b).count() as i32
}

pub fn day18_snd() -> i32 {
    let mut game = input();
    let x = game.grid.len();
    let y = game.grid[0].len();
    game.grid[0][0] = true;
    game.grid[0][y - 1] = true;
    game.grid[x - 1][0] = true;
    game.grid[x - 1][y - 1] = true;
    for _ in 0..100 {
        game.next2();
    }
    game.grid.iter().flatten().filter(|&&b| b).count() as i32
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
