use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{char, u32},
    multi::{count, many0},
    sequence::{preceded, terminated},
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day3.txt");

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
            Triangle::new(block[0][2], block[1][2], block[2][2]),
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

pub struct Day3 {
    triangles: Vec<Triangle>,
    block_triangles: Vec<Triangle>,
}

impl Solution for Day3 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let triangles = parse_input(input)
            .expect("Something went wrong while parsing")
            .1;
        let block_triangles = parse_input_blocks(input)
            .expect("Something went wrong while parsing")
            .1;
        Day3 {
            triangles,
            block_triangles,
        }
    }

    fn fst(&self) -> usize {
        self.triangles.iter().filter(|t| t.possible()).count()
    }

    fn snd(&self) -> usize {
        self.block_triangles.iter().filter(|t| t.possible()).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fst() {
        assert!(!Triangle::new(5, 10, 25).possible())
    }

    #[test]
    fn solution() {
        let day = Day3::parse(INPUT);
        assert_eq!(day.fst(), 917);
        assert_eq!(day.snd(), 1649);
    }
}
