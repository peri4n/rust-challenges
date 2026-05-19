use std::fs;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u16;
use nom::multi::separated_list1;

use crate::helper::quickunion::QuickUnion;

const INPUT_FILE: &str = "src/aoc/y2017/day12.txt";

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

fn graph() -> QuickUnion {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read input files");
    let (_, pipes) = parse_input(&content).expect("Something went wrong while parsing the input");
    let mut graph = QuickUnion::new(pipes.len());
    for pipe in pipes {
        for to in pipe.to {
            graph.connect(pipe.from as usize, to as usize);
        }
    }
    graph
}

pub fn day11_fst() -> usize {
   graph().size_of_group(0)
}

pub fn day11_snd() -> usize {
   graph().groups()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day11_fst(), 145);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day11_snd(), 207);
    }
}
