use std::{collections::{HashMap, HashSet}, fs};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::u32,
    combinator::value, multi::separated_list1, IResult, Parser, sequence::terminated,
};

use crate::helper::combinatorics::permute;

const INPUT_FILE: &str = "src/aoc/y2015/day13.txt";

type Happyness = HashMap<(String, String), i32>;

fn extract_names(happyness: &Happyness) -> HashSet<&String> {
    happyness.keys().map(|p| &p.0).collect()
}

fn input() -> Happyness {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content).expect("Should have been able to parse the input").1
}

fn parse_input(content: &str) -> IResult<&str, Happyness> {
    separated_list1(tag("\n"), parse_line)(content)
        .map(|(rest, weights)| (rest, weights.into_iter().collect()))
}

fn parse_line(line: &str) -> IResult<&str, ((String, String), i32)> {
    let (line, pearson1) = terminated(parse_person, tag(" would "))(line)?;
    let (line, change) = terminated(parse_change, tag(" happiness units by sitting next to "))(line)?;
    let (line, person2) = terminated(parse_person, tag("."))(line)?;

    Ok((line, ((pearson1.to_string(), person2.to_string()), change)))
}

fn parse_change(change: &str) -> IResult<&str, i32> {
    alt((
        value(1, tag("gain ")), 
        value(-1, tag("lose "))
        ))
    .and(u32)
    .map(|(sign, value)| sign * value as i32).parse(change)
}

fn parse_person(person: &str) -> IResult<&str, &str> {
    alpha1(person)
}

fn permute_names(names: Vec<&String>) -> Vec<Vec<&String>> {
    permute(names)
}

fn compute_happyness(happyness: &Happyness, arrangement: Vec<&String>) -> i32 {
    let mut sum = 0;
    for i in 0..arrangement.len() {
        let left = if i == 0 { arrangement.len() - 1 } else { i - 1 };
        let right = if i == arrangement.len() - 1 { 0 } else { i + 1 };

        sum += happyness.get(&(arrangement[i].clone(), arrangement[left].clone())).expect("Could not find entry");
        sum += happyness.get(&(arrangement[i].clone(), arrangement[right].clone())).expect("Could not find entry");
    }

    sum
}

pub fn day13_fst() -> i32 {
    let contents = input();
    let names = extract_names(&contents).into_iter().collect();
    let all_combinations = permute_names(names);

    let mut max_happyness = 0;
    for combination in all_combinations {
        let happyness = compute_happyness(&contents, combination);
        max_happyness = max_happyness.max(happyness);
    }

    max_happyness
}

pub fn day13_snd() -> i32 {
    let mut contents = input();
    let contents2 = contents.clone();

    let mut names = extract_names(&contents2);
    let me = "Me".to_string();

    for &name in names.iter() {
        contents.insert((me.clone(), name.clone()), 0);
        contents.insert((name.clone(), me.clone()), 0);
    }

    names.insert(&me);
    
    let all_combinations = permute_names(names.into_iter().collect());

    let mut max_happyness = 0;
    for combination in all_combinations {
        let happyness = compute_happyness(&contents, combination);
        max_happyness = max_happyness.max(happyness);
    }

    max_happyness

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day13_fst() {
        assert_eq!(day13_fst(), 733);
    }

    #[test]
    fn test_day13_snd() {
        assert_eq!(day13_snd(), 725);
    }
}
