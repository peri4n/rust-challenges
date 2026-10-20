use nom::{
    IResult,
    character::complete::{line_ending, not_line_ending},
    multi::count,
    sequence::terminated,
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day6.txt");

pub struct Day6 {
    blocks: Vec<Block>,
}

impl Solution for Day6 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let blocks = parse_blocks(input).expect("parse error").1;
        Day6 { blocks }
    }

    fn fst(&self) -> usize {
        self.blocks.iter().map(|b| b.evaluate_human()).sum()
    }

    fn snd(&self) -> usize {
        self.blocks.iter().map(|b| b.evaluate_cephalopod()).sum()
    }
}

#[derive(Debug, Clone)]
struct Block {
    rows: [String; 4],
    op: char,
}

fn parse_blocks(input: &str) -> IResult<&str, Vec<Block>> {
    let (rest, lines): (_, Vec<&str>) = count(terminated(not_line_ending, line_ending), 5)(input)?;

    // Right-pad all lines to the same width
    let max_len = lines.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut padded: Vec<Vec<char>> = (0..5).map(|_| Vec::with_capacity(max_len)).collect();
    for (i, &l) in lines.iter().enumerate() {
        let mut v: Vec<char> = l.chars().collect();
        if v.len() < max_len {
            v.resize(max_len, ' ');
        }
        padded[i] = v;
    }

    // Identify blocks by columns where all 5 are spaces
    let mut blocks = Vec::new();
    let mut c = 0;
    while c < max_len {
        // skip separator columns (all spaces across all rows)
        while c < max_len && (0..5).all(|r| padded[r][c] == ' ') {
            c += 1;
        }

        if c >= max_len {
            break;
        }

        let start = c;
        // advance until next separator column
        while c < max_len && (0..5).any(|r| padded[r][c] != ' ') {
            c += 1;
        }

        let end = c; // exclusive
        // Extract rows and operator
        let mut rows: [String; 4] = [String::new(), String::new(), String::new(), String::new()];
        for r in 0..4 {
            rows[r] = padded[r][start..end].iter().collect();
        }
        let op_slice: String = padded[4][start..end].iter().collect();
        let op = op_slice
            .chars()
            .find(|&ch| ch == '+' || ch == '*')
            .expect("missing op");
        blocks.push(Block { rows, op });
    }

    Ok((rest, blocks))
}

impl Block {
    fn numbers_normal(&self) -> [usize; 4] {
        let mut out = [0usize; 4];
        for (i, s) in self.rows.iter().enumerate() {
            let t = s.trim();
            out[i] = t.parse::<usize>().expect("bad number");
        }
        out
    }

    fn numbers_cephalopod(&self) -> Vec<usize> {
        let width = self.rows[0].len();
        assert!(self.rows.iter().all(|r| r.len() == width));
        let mut out = Vec::new();
        for col in 0..width {
            let mut s = String::new();
            for r in 0..4 {
                if let Some(ch) = self.rows[r].chars().nth(col)
                    && ch.is_ascii_digit()
                {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                out.push(s.parse::<usize>().unwrap());
            }
        }
        out
    }

    fn evaluate_human(&self) -> usize {
        let nums = self.numbers_normal();
        match self.op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => unreachable!(),
        }
    }

    fn evaluate_cephalopod(&self) -> usize {
        let nums = self.numbers_cephalopod();
        match self.op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day6::parse(INPUT);
        assert_eq!(day.fst(), 4449991244405);
        assert_eq!(day.snd(), 9348430857627);
    }
}
