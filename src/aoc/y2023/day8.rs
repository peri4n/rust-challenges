use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, newline, one_of};
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use std::collections::HashMap;

pub fn day8_fst() -> u32 {
    let content = include_str!("../y2023/day8.txt");

    let problem = parse_problem(content).unwrap().1;

    let mut node = problem.network.start();
    let mut steps = 0;

    for instruction in problem.instructions.into_iter() {
        node = problem.network.walk(node, instruction);
        steps += 1;
        if node.label == *"ZZZ" {
            break;
        }
    }

    steps
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Instructions {
    instructions: Vec<u8>,
}

impl Instructions {
    pub fn new(text: &str) -> Self {
        Self {
            instructions: text.as_bytes().to_vec(),
        }
    }
}

struct InstructionsIter<'a> {
    instructions: &'a [u8],
    index: usize,
}

impl<'a> Iterator for InstructionsIter<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.instructions[self.index % self.instructions.len()] {
            b'L' => Some(Instruction::Left),
            _ => Some(Instruction::Right),
        };

        self.index += 1;
        res
    }
}

impl<'a> IntoIterator for &'a Instructions {
    type Item = Instruction;

    type IntoIter = InstructionsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        InstructionsIter {
            instructions: &self.instructions,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    label: String,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Self { label: label.to_owned() }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Network {
    paths: HashMap<Node, (Node, Node)>,
}

impl Network {
    pub fn walk(&self, from: &Node, instruction: Instruction) -> &Node {
        match instruction {
            Instruction::Left => &self.paths.get(from).unwrap().0,
            Instruction::Right => &self.paths.get(from).unwrap().1,
        }
    }

    pub fn start(&self) -> &Node {
        self.paths.get_key_value(&Node::new("AAA")).unwrap().0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    instructions: Instructions,
    network: Network,
}

fn parse_problem(text: &str) -> IResult<&str, Problem> {
    separated_pair(parse_instructions, count(newline, 2), parse_network)
        .map(|(instructions, network)| Problem { instructions, network })
        .parse(text)
}

fn parse_instructions(line: &str) -> IResult<&str, Instructions> {
    many1(one_of("LR").map(|c| c as u8))
        .map(|instructions| Instructions { instructions })
        .parse(line)
}

fn parse_node(text: &str) -> IResult<&str, Node> {
    count(anychar.map(|c| c as u8), 3)
        .map(|label| Node {
            label: String::from_utf8_lossy(&label).into_owned(),
        })
        .parse(text)
}

fn parse_network(lines: &str) -> IResult<&str, Network> {
    separated_list1(
        newline,
        separated_pair(
            parse_node,
            tag(" = "),
            separated_pair(
                preceded(char('('), parse_node),
                tag(", "),
                terminated(parse_node, char(')')),
            ),
        ),
    )
    .map(|paths| Network {
        paths: HashMap::from_iter(paths),
    })
    .parse(lines)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_the_instructions() {
        assert_eq!(
            parse_instructions("LRLRLR").unwrap().1,
            Instructions {
                instructions: "LRLRLR".as_bytes().to_vec()
            }
        )
    }

    #[test]
    fn parsing_a_node() {
        assert_eq!(
            parse_node("AAA").unwrap().1,
            Node {
                label: String::from("AAA")
            }
        )
    }

    #[test]
    fn parsing_some_paths() {
        let text = "AAA = (BBB, CCC)\n\
            BBB = (AAA, CCC)\n\
            CCC = (DDD, CCC)";
        assert_eq!(
            parse_network(text).unwrap().1,
            Network {
                paths: HashMap::from_iter(vec![
                    (Node::new("AAA"), (Node::new("BBB"), Node::new("CCC"))),
                    (Node::new("BBB"), (Node::new("AAA"), Node::new("CCC"))),
                    (Node::new("CCC"), (Node::new("DDD"), Node::new("CCC")))
                ])
            }
        )
    }

    #[test]
    fn parsing_a_problem() {
        let text = "LLRLRLR\n\n\
                    AAA = (BBB, CCC)\n\
                    BBB = (AAA, CCC)\n\
                    CCC = (DDD, CCC)";
        assert_eq!(
            parse_problem(text).unwrap().1,
            Problem {
                instructions: Instructions {
                    instructions: "LLRLRLR".as_bytes().to_vec()
                },
                network: Network {
                    paths: HashMap::from_iter(vec![
                        (Node::new("AAA"), (Node::new("BBB"), Node::new("CCC"))),
                        (Node::new("BBB"), (Node::new("AAA"), Node::new("CCC"))),
                        (Node::new("CCC"), (Node::new("DDD"), Node::new("CCC")))
                    ])
                }
            }
        )
    }

    #[test]
    fn instruction_iterator() {
        let instructions = Instructions {
            instructions: "LLRLR".as_bytes().to_vec(),
        };
        let mut iter = instructions.into_iter();

        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Right));
        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Right));
        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Right));
        assert_eq!(iter.next(), Some(Instruction::Left));
        assert_eq!(iter.next(), Some(Instruction::Right));
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day8_fst(), 19099);
    }
}
