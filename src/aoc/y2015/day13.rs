use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::alpha1,
    character::complete::u32, combinator::value, multi::separated_list1, sequence::terminated,
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day13.txt");

type Happyness<'a> = HashMap<(&'a str, &'a str), i32>;

pub struct Day13 {
    happyness: HashMap<(String, String), i32>,
}

impl Day13 {
    fn happyness(&self) -> Happyness<'_> {
        self.happyness
            .iter()
            .map(|((p1, p2), &change)| ((p1.as_str(), p2.as_str()), change))
            .collect()
    }
}

fn extract_names<'a>(happyness: &Happyness<'a>) -> HashSet<&'a str> {
    happyness.keys().map(|p| p.0).collect()
}

fn parse_input(content: &str) -> IResult<&str, Happyness<'_>> {
    separated_list1(tag("\n"), parse_line)(content)
        .map(|(rest, weights)| (rest, weights.into_iter().collect()))
}

fn parse_line(line: &str) -> IResult<&str, ((&str, &str), i32)> {
    let (line, person1) = terminated(parse_person, tag(" would "))(line)?;
    let (line, change) =
        terminated(parse_change, tag(" happiness units by sitting next to "))(line)?;
    let (line, person2) = terminated(parse_person, tag("."))(line)?;

    Ok((line, ((person1, person2), change)))
}

fn parse_change(change: &str) -> IResult<&str, i32> {
    alt((value(1, tag("gain ")), value(-1, tag("lose "))))
        .and(u32)
        .map(|(sign, value)| sign * value as i32)
        .parse(change)
}

fn parse_person(person: &str) -> IResult<&str, &str> {
    alpha1(person)
}

fn permute_names(names: HashSet<&str>) -> Vec<Vec<&str>> {
    let len = names.len();
    names.into_iter().permutations(len).collect()
}

fn compute_happyness(happyness: &Happyness<'_>, arrangement: &[&str]) -> i32 {
    let len = arrangement.len();
    let mut sum = 0;
    for i in 0..len {
        let left = if i == 0 { len - 1 } else { i - 1 };
        let right = if i == len - 1 { 0 } else { i + 1 };

        sum += happyness
            .get(&(arrangement[i], arrangement[left]))
            .expect("Could not find entry");
        sum += happyness
            .get(&(arrangement[i], arrangement[right]))
            .expect("Could not find entry");
    }

    sum
}

fn max_happyness(happyness: &Happyness<'_>, arrangements: Vec<Vec<&str>>) -> i32 {
    arrangements
        .iter()
        .map(|c| compute_happyness(happyness, c))
        .max()
        .unwrap_or(0)
}

impl Solution for Day13 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let happyness = parse_input(input)
            .expect("Should have been able to parse the input")
            .1
            .into_iter()
            .map(|((p1, p2), change)| ((p1.to_string(), p2.to_string()), change))
            .collect();
        Day13 { happyness }
    }

    fn fst(&self) -> i32 {
        let happyness = self.happyness();
        let names = extract_names(&happyness);

        max_happyness(&happyness, permute_names(names))
    }

    fn snd(&self) -> i32 {
        let mut happyness = self.happyness();

        let mut names = extract_names(&happyness);

        for &name in &names.clone() {
            happyness.insert(("Me", name), 0);
            happyness.insert((name, "Me"), 0);
        }
        names.insert("Me");

        max_happyness(&happyness, permute_names(names))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solution() {
        let day = Day13::parse(INPUT);
        assert_eq!(day.fst(), 733);
        assert_eq!(day.snd(), 725);
    }
}
