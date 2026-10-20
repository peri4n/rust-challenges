use nom::character::complete::char;

use nom::{IResult, branch::alt, bytes::complete::tag, multi::many1, sequence::terminated};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day8.txt");

pub struct Day8 {
    operations: Vec<Op>,
}

impl Solution for Day8 {
    type Fst = usize;
    type Snd = ();

    fn parse(input: &str) -> Self {
        let operations = parse_input(input)
            .expect("Something went wrong while parsing")
            .1;
        Day8 { operations }
    }

    fn fst(&self) -> usize {
        let mut grid = Grid::new();
        for op in &self.operations {
            grid.execute(*op);
        }

        println!("Final grid state for part 2:");
        for row in 0..6 {
            for col in 0..50 {
                let c = if grid.pixels[row][col] { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }

        grid.count_lit()
    }

    fn snd(&self) {
        todo!()
    }
}

fn parse_input(content: &str) -> IResult<&str, Vec<Op>> {
    many1(terminated(parse_operation, char('\n')))(content)
}

fn parse_operation(content: &str) -> IResult<&str, Op> {
    alt((parse_rect, parse_rotate_col, parse_rotate_row))(content)
}

fn parse_rect(content: &str) -> IResult<&str, Op> {
    let (content, _) = tag("rect ")(content)?;
    let (content, width) = nom::character::complete::u32(content)?;
    let (content, _) = tag("x")(content)?;
    let (content, height) = nom::character::complete::u32(content)?;

    Ok((
        content,
        Op::Rect {
            width: width as usize,
            height: height as usize,
        },
    ))
}

fn parse_rotate_col(content: &str) -> IResult<&str, Op> {
    let (content, _) = tag("rotate column x=")(content)?;
    let (content, col) = nom::character::complete::u32(content)?;
    let (content, _) = tag(" by ")(content)?;
    let (content, count) = nom::character::complete::u32(content)?;

    Ok((
        content,
        Op::RotateCol {
            col: col as usize,
            count: count as usize,
        },
    ))
}

fn parse_rotate_row(content: &str) -> IResult<&str, Op> {
    let (content, _) = tag("rotate row y=")(content)?;
    let (content, row) = nom::character::complete::u32(content)?;
    let (content, _) = tag(" by ")(content)?;
    let (content, count) = nom::character::complete::u32(content)?;

    Ok((
        content,
        Op::RotateRow {
            row: row as usize,
            count: count as usize,
        },
    ))
}

struct Grid {
    pixels: [[bool; 50]; 6],
}

impl Grid {
    pub fn new() -> Self {
        Self { pixels: [[false; 50]; 6] }
    }

    pub fn execute(&mut self, operation: Op) {
        match operation {
            Op::Rect { width, height } => {
                for col in 0..width {
                    for row in 0..height {
                        self.pixels[row][col] = true;
                    }
                }
            }
            Op::RotateCol { col, count } => {
                let mut new_col = [false; 6];
                for row in 0..6 {
                    new_col[(row + count) % 6] = self.pixels[row][col];
                }
                for row in 0..6 {
                    self.pixels[row][col] = new_col[row];
                }
            }
            Op::RotateRow { row, count } => {
                let mut new_row = [false; 50];
                for col in 0..50 {
                    new_row[(col + count) % 50] = self.pixels[row][col];
                }
                for col in 0..50 {
                    self.pixels[row][col] = new_row[col];
                }
            }
        }
    }

    pub fn count_lit(&self) -> usize {
        let mut count = 0;
        for col in 0..50 {
            for row in 0..6 {
                if self.pixels[row][col] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Rect { width: usize, height: usize },
    RotateCol { col: usize, count: usize },
    RotateRow { row: usize, count: usize },
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day8::parse(INPUT);
        assert_eq!(day.fst(), 116);
    }
}
