use nom::IResult;
use nom::character::complete::char;
use nom::character::complete::i32;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day2.txt");

#[derive(Debug)]
struct Packet {
    width: i32,
    height: i32,
    length: i32,
}

impl Packet {
    pub fn new(width: i32, height: i32, length: i32) -> Self {
        Packet { width, height, length }
    }
    pub fn paper(&self) -> i32 {
        let all = 2 * self.width * self.height
            + 2 * self.width * self.length
            + 2 * self.length * self.height;

        all + (self.width * self.length)
            .min(self.width * self.height)
            .min(self.length * self.height)
    }

    pub fn ribbon(&self) -> i32 {
        let all = self.height * self.width * self.length;

        all + 2
            * (self.width + self.length)
                .min(self.width + self.height)
                .min(self.length + self.height)
    }
}

fn parse_line(input: &str) -> IResult<&str, Packet> {
    let (rest, length) = i32(input)?;
    let (rest, _) = char('x')(rest)?;
    let (rest, width) = i32(rest)?;
    let (rest, _) = char('x')(rest)?;
    let (rest, height) = i32(rest)?;
    Ok((rest, Packet::new(width, height, length)))
}

pub struct Day2 {
    packets: Vec<Packet>,
}

impl Solution for Day2 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let packets = input
            .lines()
            .map(|line| {
                let (_, packet) = parse_line(line).unwrap();
                packet
            })
            .collect();
        Day2 { packets }
    }

    fn fst(&self) -> i32 {
        let mut sum = 0;
        for packet in &self.packets {
            sum += packet.paper();
        }

        sum
    }

    fn snd(&self) -> i32 {
        let mut sum = 0;
        for packet in &self.packets {
            sum += packet.ribbon();
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn paper_size() {
        assert_eq!(Packet::new(2, 3, 4).paper(), 58);
        assert_eq!(Packet::new(4, 3, 2).paper(), 58);
        assert_eq!(Packet::new(2, 4, 3).paper(), 58);
        assert_eq!(Packet::new(1, 1, 10).paper(), 43);
        assert_eq!(Packet::new(10, 1, 1).paper(), 43);
        assert_eq!(Packet::new(1, 10, 1).paper(), 43);
    }

    #[test]
    fn solution() {
        let day = Day2::parse(INPUT);
        assert_eq!(day.fst(), 1586300);
        assert_eq!(day.snd(), 3737498);
    }
}
