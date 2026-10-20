use nom::{
    IResult, bytes::complete::tag, character::complete::alpha1, character::complete::u32,
    multi::separated_list1, sequence::terminated,
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day14.txt");

pub struct Day14 {
    horses: Vec<Horse>,
}

#[derive(Debug)]
struct Horse {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Horse {
    pub fn distance(&self, time: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining_time = time % cycle_time;

        let fly_time = std::cmp::min(remaining_time, self.fly_time);

        cycles * self.speed * self.fly_time + fly_time * self.speed
    }
}

impl Solution for Day14 {
    type Fst = u32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let horses = parse_input(input)
            .expect("Should have been able to parse the input")
            .1;
        Day14 { horses }
    }

    fn fst(&self) -> u32 {
        self.horses.iter().map(|h| h.distance(2503)).max().unwrap()
    }

    fn snd(&self) -> i32 {
        let horses = &self.horses;
        let mut scores = vec![0; horses.len()];

        for i in 1..=2503 {
            let distances: Vec<_> = horses.iter().map(|h| h.distance(i)).collect();
            let max_distance = distances.iter().max().unwrap();

            for (j, d) in distances.iter().enumerate() {
                if d == max_distance {
                    scores[j] += 1;
                }
            }
        }

        *scores.iter().max().unwrap()
    }
}

fn parse_input(content: &str) -> IResult<&str, Vec<Horse>> {
    separated_list1(tag("\n"), parse_horse)(content)
}

fn parse_horse(content: &str) -> IResult<&str, Horse> {
    let (content, _) = terminated(alpha1, tag(" can fly "))(content)?;
    let (content, speed) = terminated(u32, tag(" km/s for "))(content)?;
    let (content, fly_time) = terminated(u32, tag(" seconds, but then must rest for "))(content)?;
    let (content, rest_time) = terminated(u32, tag(" seconds."))(content)?;

    Ok((
        content,
        Horse {
            speed,
            fly_time,
            rest_time,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let day = Day14::parse(INPUT);
        assert_eq!(day.fst(), 2640);
        assert_eq!(day.snd(), 1102);
    }
}
