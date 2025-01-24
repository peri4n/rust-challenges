use std::fs;

use nom::{
    bytes::complete::tag, 
    character::complete::u32,
    character::complete::alpha1, 
    multi::separated_list1, 
    sequence::terminated, 
    IResult
};

const INPUT_FILE: &str = "src/aoc/y2015/day14.txt";

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

pub fn day14_fst() -> u32 {
    input().iter().map(|h| h.distance(2503)).max().unwrap()
}

pub fn day14_snd() -> i32 {
    let horses = input();
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

fn input() -> Vec<Horse> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content).expect("Should have been able to parse the input").1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Horse>> {
    separated_list1(tag("\n"), parse_horse)(content)
}

fn parse_horse(content: &str) -> IResult<&str, Horse> {
    let (content, _) = terminated(alpha1, tag(" can fly "))(content)?;
    let (content, speed) = terminated(u32, tag(" km/s for "))(content)?;
    let (content, fly_time) = terminated(u32, tag(" seconds, but then must rest for "))(content)?;
    let (content, rest_time) = terminated(u32, tag(" seconds."))(content)?;

    Ok((content, Horse { speed, fly_time, rest_time}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_fst() {
        assert_eq!(day14_fst(), 2640);
    }

    #[test]
    fn test_day14_snd() {
        assert_eq!(day14_snd(), 1102);
    }
}
