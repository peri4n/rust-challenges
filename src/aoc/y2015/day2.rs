use nom::character::complete::char;
use nom::character::complete::i32;
use nom::IResult;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "src/aoc/y2015/day2.txt";

#[derive(Debug)]
struct Packet {
    width: i32,
    height: i32,
    length: i32,
}

impl Packet {
    pub fn new(width: i32, height: i32, length: i32) -> Self {
        Packet {
            width,
            height,
            length,
        }
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
    Ok((
        rest,
        Packet::new(width, height, length)
    ))
}

fn input() -> impl Iterator<Item = Packet> {
    let file = File::open(INPUT_FILE).expect("Unable to read input file");
    io::BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();

        let (_, packet) = parse_line(&line).unwrap();
        packet
    })
}

pub fn day2_fst() -> i32 {
    let mut sum = 0;
    for packet in input() {
        sum += packet.paper();
    }

    sum
}

pub fn day2_snd() -> i32 {
    let mut sum = 0;
    for packet in input() {
        sum += packet.ribbon();
    }

    sum
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
    fn solution_fst() {
        assert_eq!(day2_fst(), 1586300);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), 3737498);
    }
}
