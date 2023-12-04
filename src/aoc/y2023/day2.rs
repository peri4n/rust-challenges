use std::fs;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::{bytes::complete::tag, sequence::delimited};

const INPUT_FILE: &str = "src/aoc/y2023/day2.txt";

fn input() -> Vec<Game> {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    parse_problem(content.as_str())
        .expect("Error parsing the input")
        .1
}

pub fn day2_fst() -> u32 {
    input()
        .into_iter()
        .filter(valid_game)
        .map(|game| game.index as u32)
        .sum()
}

struct Game {
    index: u8,
    revelations: Vec<Revelation>,
}

impl Game {
    pub fn power(&self) -> u32 {
        let red = self.revelations.iter().map(|rev| rev.red).max().unwrap_or(0) as u32;
        let green = self.revelations.iter().map(|rev| rev.green).max().unwrap_or(0) as u32;
        let blue = self.revelations.iter().map(|rev| rev.blue).max().unwrap_or(0) as u32;
        
        red * green * blue
    }
}

struct Revelation {
    red: u8,
    green: u8,
    blue: u8,
}

fn valid_game(game: &Game) -> bool {
    return game.revelations.iter().all(valid_revelation);
}

fn valid_revelation(revelation: &Revelation) -> bool {
    return revelation.red <= 12 && revelation.green <= 13 && revelation.blue <= 14;
}

fn parse_problem(text: &str) -> IResult<&str, Vec<Game>> {
    many0(terminated(parse_game, newline))(text)
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    map(
        pair(delimited(tag("Game "), u8, tag(": ")), parse_revelations),
        |(index, revelations)| Game { index, revelations },
    )(line)
}

fn parse_revelations(text: &str) -> IResult<&str, Vec<Revelation>> {
    separated_list1(tag("; "), parse_revelation)(text)
}

fn parse_revelation(text: &str) -> IResult<&str, Revelation> {
    map(
        separated_list1(tag(", "), parse_marble),
        marbles_to_revelation,
    )(text)
}

fn marbles_to_revelation(marbles: Vec<(u8, Color)>) -> Revelation {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for (count, color) in marbles {
        match color {
            Color::Red => red = count,
            Color::Green => green = count,
            Color::Blue => blue = count,
        }
    }
    Revelation { red, green, blue }
}

#[derive(Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_marble(text: &str) -> IResult<&str, (u8, Color)> {
    separated_pair(
        u8,
        char(' '),
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        )),
    )(text)
}

pub fn day2_snd() -> u32 {
    input()
        .into_iter()
        .map(|game| game.power())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day2_fst(), 2283);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), 78669);
    }
}
