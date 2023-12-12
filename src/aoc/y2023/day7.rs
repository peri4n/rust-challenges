use std::cmp::Ordering;

use nom::character::complete::alphanumeric1;
use nom::character::complete::{char, newline, u32};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

pub fn day7_fst() -> u32 {
    let content = include_str!("../y2023/day7.txt");

    let mut cards = parse_cards(content).unwrap().1;
    day7_fst_solve(&mut cards)
}

fn day7_fst_solve<'a>(hands: &mut Vec<Hand<'a>>) -> u32 {
    hands.sort();

    let mut sum = 0;
    for i in 0..hands.len() {
        sum += (i as u32 + 1) * hands[i].bid;
    }
    sum
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_rank = ranking(self.cards);
        let other_rank = ranking(other.cards);

        if self_rank == other_rank {
            return Some(first_high_card(&self.cards, &other.cards));
        }

        Some(self_rank.cmp(&other_rank))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn first_high_card(cards_1: &str, cards_2: &str) -> std::cmp::Ordering {
    let c1 = cards_1.chars().collect::<Vec<_>>();
    let c2 = cards_2.chars().collect::<Vec<_>>();

    for i in 0..c1.len() {
        if index(c1[i]) > index(c2[i]) {
            return Ordering::Greater;
        }
        if index(c1[i]) < index(c2[i]) {
            return Ordering::Less;
        }
    }
    std::cmp::Ordering::Equal
}

fn parse_cards<'a>(text: &'a str) -> IResult<&str, Vec<Hand<'a>>> {
    separated_list1(newline, parse_hand)(text)
}

fn parse_hand<'a>(line: &'a str) -> IResult<&str, Hand<'a>> {
    separated_pair(alphanumeric1, char(' '), u32)
        .map(|(cards, bid)| Hand { cards, bid })
        .parse(line)
}

const CARD_RANGE: usize = 13;

fn index(c: char) -> u8 {
    (match c {
        '2'..='9' => (c as u8) & 15,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        x => panic!("Should not happen {}", x),
    }) - 2
}

fn ranking(hand: &str) -> Ranking {
    let mut counts = [0; CARD_RANGE];
    for c in hand.chars() {
        counts[index(c) as usize] += 1;
    }

    [
        is_five_of_a_kind(&counts),
        is_four_of_a_kind(&counts),
        is_fullhouse(&counts),
        is_three_of_a_kind(&counts),
        is_two_pair(&counts),
        is_pair(&counts),
        is_high_card(&counts),
    ]
    .into_iter()
    .find(|ranking| ranking.is_some())
    .expect("You will always find a ranking")
    .expect("There will always be a mapping")
}

fn find(counts: &[u8], num: u8) -> bool {
    for count in counts {
        if *count == num {
            return true;
        }
    }
    false
}

fn find2(counts: &[u8], num1: u8, num2: u8) -> bool {
    let mut found_num1 = false;
    let mut found_num2 = false;

    for count in counts {
        if *count == num1 && !found_num1 {
            found_num1 = true;
            continue;
        }
        if *count == num2 {
            found_num2 = true;
        }
    }
    found_num1 && found_num2
}

fn is_five_of_a_kind(counts: &[u8]) -> Option<Ranking> {
    if find(counts, 5) {
        return Some(Ranking::FiveOfAKind);
    }
    None
}

fn is_four_of_a_kind(counts: &[u8]) -> Option<Ranking> {
    if find(counts, 4) {
        return Some(Ranking::FourOfAKind);
    }
    None
}

fn is_fullhouse(counts: &[u8]) -> Option<Ranking> {
    if find2(counts, 3, 2) {
        return Some(Ranking::FullHouse);
    }
    None
}

fn is_three_of_a_kind(counts: &[u8]) -> Option<Ranking> {
    if find(counts, 3) {
        return Some(Ranking::ThreeOfAKind);
    }
    None
}

fn is_two_pair(counts: &[u8]) -> Option<Ranking> {
    if find2(counts, 2, 2) {
        return Some(Ranking::TwoPair);
    }
    None
}

fn is_pair(counts: &[u8]) -> Option<Ranking> {
    if find(counts, 2) {
        return Some(Ranking::Pair);
    }
    None
}

fn is_high_card(counts: &[u8]) -> Option<Ranking> {
    for i in (0..CARD_RANGE).rev() {
        if counts[i] != 0 {
            return Some(Ranking::HighCard);
        }
    }
    None // impossibuuuu
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
#[repr(u8)]
enum Ranking {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcases() {
        assert_eq!(ranking("82T3A"), Ranking::HighCard);
        assert_eq!(ranking("32T3K"), Ranking::Pair);
        assert_eq!(ranking("3233K"), Ranking::ThreeOfAKind);
        assert_eq!(ranking("KK677"), Ranking::TwoPair);
        assert_eq!(ranking("T55T5"), Ranking::FullHouse);
        assert_eq!(ranking("3333K"), Ranking::FourOfAKind);
        assert_eq!(ranking("33333"), Ranking::FiveOfAKind);
    }

    #[test]
    fn ordering() {
        assert!(Ranking::Pair > Ranking::HighCard);
        assert!(Ranking::TwoPair > Ranking::Pair);
        assert!(Ranking::ThreeOfAKind > Ranking::TwoPair);
        assert!(Ranking::FullHouse > Ranking::ThreeOfAKind);
        assert!(Ranking::FourOfAKind > Ranking::FullHouse);
        assert!(Ranking::FiveOfAKind > Ranking::FourOfAKind);

        assert!(
            Hand { cards: "82T3A", bid: 1 } < Hand { cards: "T2J3A", bid: 1 }
        );
        assert!(
            Hand { cards: "82T3A", bid: 1 } < Hand { cards: "T2J3A", bid: 1 }
        );
        assert!(
            Hand { cards: "8TT3A", bid: 1 } == Hand { cards: "8TT3A", bid: 1 }
        );
        assert!(
            Hand { cards: "8TT3A", bid: 1 } > Hand { cards: "T2J3A", bid: 1 }
        );
        assert!(
            Hand { cards: "33332", bid: 1 } > Hand { cards: "2AAAA", bid: 1 }
        );
        assert!(
            Hand { cards: "77888", bid: 1 } < Hand { cards: "88877", bid: 1 }
        );
        assert!(
            Hand { cards: "77888", bid: 1 } > Hand { cards: "77788", bid: 1 }
        );
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day7_fst_solve(&mut vec![
                        Hand { cards: "32T3K", bid: 765 },
                        Hand { cards: "T55J5", bid: 684 },
                        Hand { cards: "KK677", bid: 28 },
                        Hand { cards: "KTJJT", bid: 220 },
                        Hand { cards: "QQQJA", bid: 483 },
        ]), 6440);

        assert_eq!(day7_fst(), 246272322);
    }
}
