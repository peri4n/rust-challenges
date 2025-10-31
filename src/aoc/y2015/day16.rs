use std::collections::HashMap;
use std::fs;

use nom::sequence::{preceded, terminated};

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, i32},
    multi::separated_list1,
};

const INPUT_FILE: &str = "src/aoc/y2015/day16.txt";

fn the_real_sue() -> Properties {
    let mut the_real_sue = HashMap::new();
    the_real_sue.insert("children".to_string(), 3);
    the_real_sue.insert("cats".to_string(), 7);
    the_real_sue.insert("samoyeds".to_string(), 2);
    the_real_sue.insert("pomeranians".to_string(), 3);
    the_real_sue.insert("akitas".to_string(), 0);
    the_real_sue.insert("vizslas".to_string(), 0);
    the_real_sue.insert("goldfish".to_string(), 5);
    the_real_sue.insert("trees".to_string(), 3);
    the_real_sue.insert("cars".to_string(), 2);
    the_real_sue.insert("perfumes".to_string(), 1);
    the_real_sue
}

pub fn day16_fst() -> i32 {
    let the_real_sue = the_real_sue();
    let aunts = input();
    aunts
        .iter()
        .find(|aunt| aunt.matches(&the_real_sue))
        .unwrap()
        .number
}

pub fn day16_snd() -> i32 {
    let the_real_sue = the_real_sue();
    let aunts = input();
    aunts
        .iter()
        .find(|aunt| aunt.matches2(&the_real_sue))
        .unwrap()
        .number
}

fn input() -> Vec<Aunt> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Should have been able to parse the input")
        .1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Aunt>> {
    separated_list1(tag("\n"), parse_aunt)(content)
}

fn parse_aunt(content: &str) -> IResult<&str, Aunt> {
    let (content, number) = terminated(preceded(tag("Sue "), digit1), tag(": "))(content)?;
    let (content, properties) = separated_list1(tag(", "), parse_property)(content)?;

    Ok((
        content,
        Aunt {
            number: number.parse().unwrap(),
            properties: properties.into_iter().collect(),
        },
    ))
}

#[derive(Debug)]
struct Aunt {
    number: i32,
    properties: Properties,
}

impl Aunt {
    pub fn matches(&self, properties: &Properties) -> bool {
        for (name, value) in properties {
            if let Some(v) = self.properties.get(name) {
                if v != value {
                    return false;
                }
            }
        }
        true
    }

    pub fn matches2(&self, properties: &Properties) -> bool {
        for (name, value) in properties {
            if let Some(v) = self.properties.get(name) {
                match name.as_str() {
                    "cats" | "trees" => {
                        if v <= value {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if v >= value {
                            return false;
                        }
                    }
                    _ => {
                        if v != value {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

type Properties = HashMap<String, i32>;

fn parse_property(content: &str) -> IResult<&str, (String, i32)> {
    let (content, name) = terminated(alpha1, tag(": "))(content)?;
    let (content, value) = i32(content)?;

    Ok((content, (name.to_string(), value)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day16_fst() {
        assert_eq!(day16_fst(), 103);
    }

    #[test]
    fn test_day16_snd() {
        assert_eq!(day16_snd(), 405);
    }
}
