use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    multi::{count, many0},
    sequence::{preceded, terminated},
    IResult,
};

const INPUT_FILE: &str = "src/aoc/y2016/day3.txt";

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    pub fn possible(&self) -> bool {
        self.c < self.a + self.b && self.a < self.b + self.c && self.b < self.a + self.c
    }
}

fn input() -> Vec<Triangle> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Something went wrong while parsing")
        .1
}

fn input_blocks() -> Vec<Triangle> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input_blocks(&content)
        .expect("Something went wrong while parsing")
        .1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Triangle>> {
    many0(terminated(parse_triangle, tag("\n")))(content)
}

fn parse_input_blocks(content: &str) -> IResult<&str, Vec<Triangle>> {
    let (content, blocks) = many0(parse_input_block)(content)?;

    Ok((content, blocks.into_iter().flatten().collect()))
}

fn parse_input_block(content: &str) -> IResult<&str, Vec<Triangle>> {
    let (content, block) = count(terminated(count(parse_length, 3), tag("\n")), 3)(content)?;

    Ok((
        content,
        vec![
            Triangle::new(block[0][0], block[1][0], block[2][0]),
            Triangle::new(block[0][1], block[1][1], block[2][1]),
            Triangle::new(block[0][2], block[1][2], block[2][2])
        ],
    ))
}

fn parse_triangle(content: &str) -> IResult<&str, Triangle> {
    let (content, res) = count(parse_length, 3)(content)?;

    Ok((
        content,
        Triangle {
            a: res[0],
            b: res[1],
            c: res[2],
        },
    ))
}

fn parse_length(content: &str) -> IResult<&str, u32> {
    preceded(many0(char(' ')), u32)(content)
}

pub fn day3_fst() -> usize {
    input().into_iter().filter(|t| t.possible()).count()
}

pub fn day3_snd() -> usize {
    input_blocks().into_iter().filter(|t| t.possible()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fst() {
        assert!(!Triangle::new(5, 10, 25).possible())
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day3_fst(), 917);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day3_snd(), 1649);
    }
}
