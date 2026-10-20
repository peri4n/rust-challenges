use std::collections::VecDeque;

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day4.txt");

#[derive(Debug, Clone)]
struct ScratchCard {
    index: u8,
    cards: Vec<u8>,
    winners: Vec<u8>,
}

impl ScratchCard {
    pub fn score(&self) -> u32 {
        let matches = self
            .cards
            .iter()
            .filter(|&n| self.winners.contains(n))
            .count() as u32;

        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }

    fn won_card_indexes(&self) -> Vec<u8> {
        let mut n = 0;
        for x in &self.cards {
            if self.winners.contains(x) {
                n += 1;
            }
        }
        (0..n).map(|i| self.index + i).collect()
    }

    pub fn won_cards<'a>(&'a self, cards: &'a [ScratchCard]) -> Vec<&'a ScratchCard> {
        self.won_card_indexes()
            .iter()
            .map(|id| cards.get(*id as usize).unwrap())
            .collect()
    }
}

pub struct Day4 {
    cards: Vec<ScratchCard>,
}

impl Solution for Day4 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let cards = parse_problem(input).expect("Error parsing the input").1;
        Day4 { cards }
    }

    fn fst(&self) -> u32 {
        self.cards.iter().map(|card| card.score()).sum()
    }

    fn snd(&self) -> u32 {
        let cards = &self.cards;
        let mut que: VecDeque<&ScratchCard> = VecDeque::with_capacity(cards.len());
        que.extend(cards.as_slice());

        let mut count = 0;
        while let Some(card) = que.pop_front() {
            que.extend(card.won_cards(cards));
            count += 1;
        }

        count
    }
}

fn parse_problem(text: &str) -> IResult<&str, Vec<ScratchCard>> {
    separated_list1(newline, parse_card)(text)
}

fn parse_card(line: &str) -> IResult<&str, ScratchCard> {
    delimited(
        tag("Card").and(many1(char(' '))),
        u8,
        tag(":").and(many1(char(' '))),
    )
    .and(separated_pair(
        parse_cards,
        separated_pair(many1(char(' ')), char('|'), many1(char(' '))),
        parse_winners,
    ))
    .map(|(index, (cards, winners))| ScratchCard { index, cards, winners })
    .parse(line)
}

fn parse_cards(text: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(many1(char(' ')), u8)(text)
}

fn parse_winners(text: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(many1(char(' ')), u8)(text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcases() {
        assert_eq!(
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .1
                .score(),
            8
        );
        assert_eq!(
            parse_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
                .unwrap()
                .1
                .score(),
            2
        );
        assert_eq!(
            parse_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
                .unwrap()
                .1
                .score(),
            2
        );
        assert_eq!(
            parse_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
                .unwrap()
                .1
                .score(),
            1
        );
        assert_eq!(
            parse_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
                .unwrap()
                .1
                .score(),
            0
        );
        assert_eq!(
            parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
                .unwrap()
                .1
                .score(),
            0
        );
    }

    #[test]
    fn solution() {
        let day = Day4::parse(INPUT);
        assert_eq!(day.fst(), 20667);
        assert_eq!(day.snd(), 5833065);
    }
}
