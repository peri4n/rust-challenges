use nom::{
    IResult, bytes::complete::tag, character::complete::alphanumeric1, multi::many1,
    sequence::terminated,
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day6.txt");

pub struct Day6 {
    lines: Vec<String>,
}

impl Solution for Day6 {
    type Fst = String;
    type Snd = String;

    fn parse(input: &str) -> Self {
        let lines = parse_input(input)
            .expect("Something went wrong while parsing")
            .1
            .into_iter()
            .map(String::from)
            .collect();
        Day6 { lines }
    }

    fn fst(&self) -> String {
        let frequencies = compute_frequencies(&self.lines);

        let mut res = String::new();
        for f in frequencies {
            let max_index = index_of_min_max(&f).1;
            res.push((max_index + (b'a')) as char)
        }

        res
    }

    fn snd(&self) -> String {
        let frequencies = compute_frequencies(&self.lines);

        let mut res = String::new();
        for f in frequencies {
            let min_index = index_of_min_max(&f).0;
            res.push((min_index + (b'a')) as char)
        }

        res
    }
}

fn compute_frequencies(words: &[String]) -> [[u32; 26]; 8] {
    let mut frequencies = [[0; 26]; 8];

    for word in words {
        for (i, c) in word.char_indices() {
            frequencies[i][c as usize - 'a' as usize] += 1;
        }
    }

    frequencies
}

fn index_of_min_max(frequencies: &[u32]) -> (u8, u8) {
    let mut min_index = 0;
    let mut current_min = u32::MAX;

    let mut max_index = 0;
    let mut current_max = u32::MIN;

    for (i, &f) in frequencies.iter().enumerate() {
        if f < current_min {
            current_min = f;
            min_index = i;
        }
        if f > current_max {
            current_max = f;
            max_index = i;
        }
    }

    (min_index as u8, max_index as u8)
}

fn parse_input(content: &str) -> IResult<&str, Vec<&str>> {
    many1(terminated(alphanumeric1, tag("\n")))(content)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day6::parse(INPUT);
        assert_eq!(day.fst(), String::from("tsreykjj"));
        assert_eq!(day.snd(), String::from("hnfbujie"));
    }
}
