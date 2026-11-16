use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::u32,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day3.txt");

const SIZE: usize = 1000;

pub struct Day3 {
    // Only 0, 1 and "2 or more" matter, so u8 with saturating adds is enough.
    fabric: Vec<[u8; SIZE]>,
    claims: Vec<Claim>,
    overlaps: usize,
}

pub struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.top..self.top + self.height)
            .flat_map(move |y| (self.left..self.left + self.width).map(move |x| (y, x)))
    }
}

impl Day3 {
    fn claim(&mut self, claim: &Claim) {
        for (y, x) in claim.cells() {
            let cell = &mut self.fabric[y][x];
            if *cell == 1 {
                self.overlaps += 1;
            }
            *cell = cell.saturating_add(1);
        }
    }
}

impl Solution for Day3 {
    type Fst = usize;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let claims = parse_lines(input).expect("Unable to parse claims").1;
        let mut day = Self {
            fabric: vec![[0; SIZE]; SIZE],
            claims: Vec::new(),
            overlaps: 0,
        };
        for claim in &claims {
            day.claim(claim);
        }
        day.claims = claims;
        day
    }

    fn fst(&self) -> Self::Fst {
        self.overlaps
    }

    fn snd(&self) -> Self::Snd {
        self.claims
            .iter()
            .find(|claim| claim.cells().all(|(y, x)| self.fabric[y][x] == 1))
            .expect("No intact claim")
            .id
    }
}

fn parse_lines(content: &str) -> IResult<&str, Vec<Claim>> {
    separated_list1(tag("\n"), parse_line)(content)
}

fn parse_line(content: &str) -> IResult<&str, Claim> {
    let (rest, id) = preceded(tag("#"), u32)(content)?;
    let (rest, _) = tag(" @ ")(rest)?;
    let (rest, (left, top)) = separated_pair(u32, tag(","), u32)(rest)?;
    let (rest, _) = tag(": ")(rest)?;
    let (rest, (width, height)) = separated_pair(u32, tag("x"), u32)(rest)?;
    Ok((
        rest,
        Claim {
            id,
            left: left as usize,
            top: top as usize,
            width: width as usize,
            height: height as usize,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let day = Day3::parse("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(day.fst(), 4);
        assert_eq!(day.snd(), 3);
    }

    #[test]
    fn solution() {
        let day = Day3::parse(INPUT);
        assert_eq!(day.fst(), 107820);
        assert_eq!(day.snd(), 661);
    }
}
