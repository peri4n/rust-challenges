use std::fs;

use nom::{character::complete::newline, multi::separated_list1, IResult};

const INPUT_FILE: &str = "src/aoc/y2025/day7.txt";

pub fn day7_fst() -> u32 {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let (mut simulation, splitters) = parse_simulation(&content).expect("Failed to parse simulation").1;

    for splitter in splitters {
        for pos in splitter {
            simulation.split_at(pos);
        }
    }
    simulation.split_count
}

pub fn day7_snd() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let (mut simulation, splitters) = parse_simulation(&content).expect("Failed to parse simulation").1;

    for splitter in splitters {
        for pos in splitter {
            simulation.split_at_quantum(pos);
        }
    }
    simulation.beams_sum()
}

fn parse_simulation(content: &str) -> IResult<&str, (Simulation, Vec<Vec<usize>>)> {
    let (rest, initial_state) = parse_initial_state(content)?;
    let (rest, splitters) = parse_splitters(rest)?;
    Ok((rest, (initial_state, splitters)))
}

fn parse_splitters(rest: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(newline, parse_splits)(rest)
}

fn parse_splits(input: &str) -> IResult<&str, Vec<usize>> {
    let (rest, line) = nom::bytes::complete::take_until("\n")(input)?;
    let positions = line.match_indices('^').map(|p| p.0).collect();
    Ok((rest, positions))
}


fn parse_initial_state(content: &str) -> IResult<&str, Simulation> {
    // fetch first line
    let (rest, line) = nom::bytes::complete::take_until("\n")(content)?;
    let pos = line.find('S').expect("No starting position found");

    let mut init_beams = vec![0; 141];
    init_beams[pos] = 1;
    Ok((rest, Simulation { beams: init_beams, split_count: 0 }))
}



struct Simulation {
    beams: Vec<u64>,
    split_count: u32,
}

impl Simulation {
    fn split_at(&mut self, pos: usize) {
        if self.beams[pos] == 1 {
            self.beams[pos] = 0;
            if pos > 0 {
                self.beams[pos - 1] = 1;
            }
            if pos + 1 < self.beams.len() {
                self.beams[pos + 1] = 1;
            }
            self.split_count += 1;
        }
    }

    fn split_at_quantum(&mut self, pos: usize) {
        if self.beams[pos] > 0 {
            let count = self.beams[pos];
            self.beams[pos] = 0;
            if pos > 0 {
                self.beams[pos - 1] += count;
            }
            if pos + 1 < self.beams.len() {
                self.beams[pos + 1] += count;
            }
            self.split_count += 1
        }
    }

    fn beams_sum(&self) -> u64 {
        self.beams.iter().sum()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day7_fst(), 1619);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day7_snd(), 23607984027985);
    }
}
