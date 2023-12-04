use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

const INPUT_FILE: &str = "src/aoc/y2023/day4.txt";

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
            .filter(|&n| self.winners.iter().any(|&w| w == *n))
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

    pub fn won_cards<'a>(&'a self, cards: &'a [ScratchCard]) -> Vec<&ScratchCard> {
        self.won_card_indexes()
            .iter()
            .map(|id| cards.get(*id as usize).unwrap())
            .collect()
    }
}

fn input() -> Vec<ScratchCard> {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    parse_problem(content.as_str())
        .expect("Error parsing the input")
        .1
}

pub fn day4_fst() -> u32 {
    input().iter().map(|card| card.score()).sum()
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
    .map(|(index, (cards, winners))| ScratchCard {
        index,
        cards,
        winners,
    })
    .parse(line)
}

fn parse_cards(text: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(many1(char(' ')), u8)(text)
}

fn parse_winners(text: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(many1(char(' ')), u8)(text)
}

pub fn day4_snd() -> u32 {
    let cards = input();
    let mut que: VecDeque<&ScratchCard> = VecDeque::new();
    let mut count: HashMap<u8, u32> = HashMap::new();
    for original_card in &cards {
        que.push_back(original_card);
        while let Some(card) = que.pop_front() {
            count.entry(card.index).and_modify(|v| *v += 1).or_insert(1);

            let won_cards = card.won_cards(&cards);
            for c in won_cards {
                que.push_back(c);
            }
        }
    }

    count.values().sum()
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
    fn solution_fst() {
        assert_eq!(day4_fst(), 20667);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day4_snd(), 5833065);
    }
}
