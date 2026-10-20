use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u16;
use nom::multi::separated_list1;

use crate::aoc::Solution;
use crate::helper::quickunion::QuickUnion;

pub const INPUT: &str = include_str!("day12.txt");

#[derive(Debug)]
struct Pipes {
    from: u16,
    to: Vec<u16>,
}

fn parse_input(content: &str) -> IResult<&str, Vec<Pipes>> {
    separated_list1(newline, parse_line)(content)
}

fn parse_line(line: &str) -> IResult<&str, Pipes> {
    let (rest, from) = u16(line)?;
    let (rest, _) = tag(" <-> ")(rest)?;
    let (rest, tos) = separated_list1(tag(", "), u16)(rest)?;

    Ok((rest, Pipes { from, to: tos }))
}

fn graph(pipes: &[Pipes]) -> QuickUnion {
    let mut graph = QuickUnion::new(pipes.len());
    for pipe in pipes {
        for &to in &pipe.to {
            graph.connect(pipe.from as usize, to as usize);
        }
    }
    graph
}

pub struct Day12 {
    pipes: Vec<Pipes>,
}

impl Solution for Day12 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let (_, pipes) = parse_input(input).expect("Something went wrong while parsing the input");
        Day12 { pipes }
    }

    fn fst(&self) -> usize {
        graph(&self.pipes).size_of_group(0)
    }

    fn snd(&self) -> usize {
        graph(&self.pipes).groups()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day12::parse(INPUT);
        assert_eq!(day.fst(), 145);
        assert_eq!(day.snd(), 207);
    }
}
