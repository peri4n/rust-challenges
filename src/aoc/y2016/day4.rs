use std::{cmp::Reverse, collections::BinaryHeap, fs};

use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::all_consuming,
    multi::{many0, separated_list1},
    sequence::{preceded, terminated},
};

const INPUT_FILE: &str = "src/aoc/y2016/day4.txt";

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    pub fn new(encrypted_name: String, sector_id: u32, checksum: String) -> Self {
        Self {
            encrypted_name,
            sector_id,
            checksum,
        }
    }

    pub fn valid(&self) -> bool {
        let counts = self.encrypted_name.chars().filter(|c| *c != '-').counts();

        let mut freq: BinaryHeap<(usize, Reverse<char>)> =
            counts.into_iter().map(|(c, n)| (n, Reverse(c))).collect();

        let mut checksum = String::new();

        for _ in 0..self.checksum.len() {
            checksum.push(freq.pop().unwrap().1.0);
        }
        checksum == self.checksum
    }

    pub fn decrypted_name(&self) -> String {
        let n = 26u8;
        let offset = b'a';
        let shift = (self.sector_id % n as u32) as u8;
        let mut res = String::with_capacity(self.encrypted_name.len());

        for c in self.encrypted_name.chars() {
            if c == '-' {
                res.push(' ');
            } else {
                let i = (c as u8) - offset;
                let j = (i + shift) % n + offset;
                res.push(j as char);
            }
        }
        res
    }
}

fn input() -> Vec<Room> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Something went wrong while parsing")
        .1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Room>> {
    all_consuming(many0(terminated(parse_line, tag("\n"))))(content)
}

fn parse_line(line: &str) -> IResult<&str, Room> {
    let (line, segments) = separated_list1(tag("-"), alphanumeric1)(line)?;
    let (line, checksum) = preceded(tag("["), terminated(alpha1, tag("]")))(line)?;

    let room = Room::new(
        build_name(&segments),
        segments.last().unwrap().parse::<u32>().unwrap(),
        String::from(checksum),
    );

    Ok((line, room))
}

fn build_name(segments: &[&str]) -> String {
    let mut res = String::new();

    for (i, &seg) in segments.iter().enumerate() {
        if i < segments.len() - 1 {
            res.push_str(seg);
        }
        if i < segments.len() - 2 {
            res.push('-');
        }
    }
    res
}

pub fn day4_fst() -> u32 {
    input()
        .into_iter()
        .filter(|room| room.valid())
        .map(|room| room.sector_id)
        .sum()
}

pub fn day4_snd() -> u32 {
    input()
        .into_iter()
        .find(|room| room.decrypted_name() == "northpole object storage")
        .unwrap()
        .sector_id
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fst() {
        let room = Room::new(String::from("aaaaabbbzyx"), 123, String::from("abxyz"));
        assert!(room.valid());

        let room = Room::new(String::from("abcdefgh"), 987, String::from("abcde"));
        assert!(room.valid());

        let room = Room::new(String::from("notarealroom"), 404, String::from("oarel"));
        assert!(room.valid());

        let room = Room::new(String::from("totallyrealroom"), 200, String::from("decoy"));
        assert!(!room.valid());
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day4_fst(), 278221);
    }

    #[test]
    fn example_snd() {
        let room = Room::new(
            String::from("qzmt-zixmtkozy-ivhz"),
            343,
            String::from("ignore"),
        );

        assert_eq!(room.decrypted_name(), String::from("very encrypted name"));
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day4_snd(), 267);
    }
}
