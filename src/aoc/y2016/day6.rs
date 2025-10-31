use std::fs;

use nom::{
    IResult, bytes::complete::tag, character::complete::alphanumeric1, multi::many1,
    sequence::terminated,
};

const INPUT_FILE: &str = "src/aoc/y2016/day6.txt";

pub fn day6_fst() -> String {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let lines = parse_input(&content)
        .expect("Something went wrong while parsing")
        .1;

    let frequencies = compute_frequencies(lines);

    let mut res = String::new();
    for f in frequencies {
        let max_index = index_of_min_max(&f).1;
        res.push((max_index + (b'a')) as char)
    }

    res
}

pub fn day6_snd() -> String {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let lines = parse_input(&content)
        .expect("Something went wrong while parsing")
        .1;

    let frequencies = compute_frequencies(lines);

    let mut res = String::new();
    for f in frequencies {
        let min_index = index_of_min_max(&f).0;
        res.push((min_index + (b'a')) as char)
    }

    res
}

fn compute_frequencies(words: Vec<&str>) -> [[u32; 26]; 8] {
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
    fn solution_fst() {
        assert_eq!(day6_fst(), String::from("tsreykjj"));
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day6_snd(), String::from("hnfbujie"));
    }
}
