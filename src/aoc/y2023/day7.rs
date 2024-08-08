use std::cmp::Ordering;

use nom::character::complete::alphanumeric1;
use nom::character::complete::{char, newline, u32};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

const CARD_RANGE: usize = 13;

/**
 * Solutions
 */

pub fn day7_fst() -> u32 {
    let content = include_str!("../y2023/day7.txt");

    let mut cards = parse_cards(content).unwrap().1;
    day7_fst_solve(&mut cards)
}

fn day7_fst_solve(hands: &mut [Hand<'_>]) -> u32 {
    hands.sort_by(|a, b| compare(a.cards, b.cards));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
    }
    sum
}

pub fn day7_snd() -> u32 {
    let content = include_str!("../y2023/day7.txt");

    let mut cards = parse_cards(content).unwrap().1;
    day7_snd_solve(&mut cards)
}

fn day7_snd_solve(hands: &mut [Hand<'_>]) -> u32 {
    hands.sort_by(|a, b| compare2(a.cards, b.cards));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
    }
    sum
}


/**
 * Parsing
 */

fn parse_cards(text: &str) -> IResult<&str, Vec<Hand<'_>>> {
    separated_list1(newline, parse_hand)(text)
}

fn parse_hand(line: &str) -> IResult<&str, Hand<'_>> {
    separated_pair(alphanumeric1, char(' '), u32)
        .map(|(cards, bid)| Hand::new(cards, bid))
        .parse(line)
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

impl<'a> Hand<'a> {
    pub fn new(cards: &'a str, bid: u32) -> Self {
        Hand { cards, bid }
    }
}

/**
 * Sorting
 */

fn compare(cards_1: &str, cards_2: &str) -> std::cmp::Ordering {
    let ranking_1 = Counts::from(cards_1).ranking();
    let ranking_2 = Counts::from(cards_2).ranking();

    match ranking_1.cmp(&ranking_2) {
        Ordering::Equal => first_high_card(cards_1, cards_2),
        x => x,
    }
}

fn compare2(cards_1: &str, cards_2: &str) -> std::cmp::Ordering {
    let ranking_1 = Counts::from_with_wildcards(cards_1).ranking();
    let ranking_2 = Counts::from_with_wildcards(cards_2).ranking();

    match ranking_1.cmp(&ranking_2) {
        Ordering::Equal => first_high_card2(cards_1, cards_2),
        x => x,
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

fn first_high_card2(cards_1: &str, cards_2: &str) -> std::cmp::Ordering {
    let c1 = cards_1.chars().collect::<Vec<_>>();
    let c2 = cards_2.chars().collect::<Vec<_>>();

    for i in 0..c1.len() {
        if index2(c1[i]) > index2(c2[i]) {
            return Ordering::Greater;
        }
        if index2(c1[i]) < index2(c2[i]) {
            return Ordering::Less;
        }
    }
    std::cmp::Ordering::Equal
}

fn index2(c: char) -> u8 {
    match c {
        '2'..='9' => ((c as u8) & 15) - 1,
        'T' => 9,
        'J' => 0,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        x => panic!("Should not happen {}", x),
    }
}

/**
 * Ranking
 */

struct Counts {
    counts: [u8; CARD_RANGE],
}

impl Counts {
    pub fn from(cards: &str) -> Self {
        let mut counts = [0; CARD_RANGE];
        for c in cards.chars() {
            counts[index(c) as usize] += 1;
        }
        Self { counts }
    }

    pub fn from_with_wildcards(cards: &str) -> Self {
        let mut counts = [0; CARD_RANGE];


        let mut jester_count = 0;
        for c in cards.chars() {
            if c == 'J' {
                jester_count += 1;
            } else {
                counts[index(c) as usize] += 1;
            }
        }

        while jester_count > 0 {
            let (i, max) = counts.iter().enumerate().max_by_key(|c| *c.1).unwrap();
            let fill = (5 - max).min(jester_count);
            counts[i] += fill;

            jester_count -= fill;
        }

        Self { counts }
    }

    fn find(&self, num: u8) -> bool {
        for count in self.counts {
            if count == num {
                return true;
            }
        }
        false
    }

    fn find2(&self, num1: u8, num2: u8) -> bool {
        let mut found_num1 = false;
        let mut found_num2 = false;

        for count in self.counts {
            if count == num1 && !found_num1 {
                found_num1 = true;
                continue;
            }
            if count == num2 {
                found_num2 = true;
            }
        }
        found_num1 && found_num2
    }

    fn is_five_of_a_kind(&self) -> Option<Ranking> {
        if self.find(5) {
            return Some(Ranking::FiveOfAKind);
        }
        None
    }

    fn is_four_of_a_kind(&self) -> Option<Ranking> {
        if self.find(4) {
            return Some(Ranking::FourOfAKind);
        }
        None
    }

    fn is_fullhouse(&self) -> Option<Ranking> {
        if self.find2(3, 2) {
            return Some(Ranking::FullHouse);
        }
        None
    }

    fn is_three_of_a_kind(&self) -> Option<Ranking> {
        if self.find(3) {
            return Some(Ranking::ThreeOfAKind);
        }
        None
    }

    fn is_two_pair(&self) -> Option<Ranking> {
        if self.find2(2, 2) {
            return Some(Ranking::TwoPair);
        }
        None
    }

    fn is_pair(&self) -> Option<Ranking> {
        if self.find(2) {
            return Some(Ranking::Pair);
        }
        None
    }

    fn is_high_card(&self) -> Option<Ranking> {
        for i in (0..CARD_RANGE).rev() {
            if self.counts[i] != 0 {
                return Some(Ranking::HighCard);
            }
        }
        None // impossibuuuu
    }

    pub fn ranking(&self) -> Ranking {
        [
            self.is_five_of_a_kind(),
            self.is_four_of_a_kind(),
            self.is_fullhouse(),
            self.is_three_of_a_kind(),
            self.is_two_pair(),
            self.is_pair(),
            self.is_high_card(),
        ]
        .into_iter()
        .find(|ranking| ranking.is_some())
        .expect("You will always find a ranking")
        .expect("There will always be a mapping")
    }
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

    impl Ranking {
        fn from(cards: &str) -> Self {
            Counts::from(cards).ranking()
        }

        fn from_with_wildcards(cards: &str) -> Self {
            Counts::from_with_wildcards(cards).ranking()
        }
    }

    #[test]
    fn testcases() {
        assert_eq!(Ranking::from("82T3A"), Ranking::HighCard);
        assert_eq!(Ranking::from("32T3K"), Ranking::Pair);
        assert_eq!(Ranking::from("3233K"), Ranking::ThreeOfAKind);
        assert_eq!(Ranking::from("KK677"), Ranking::TwoPair);
        assert_eq!(Ranking::from("T55T5"), Ranking::FullHouse);
        assert_eq!(Ranking::from("3333K"), Ranking::FourOfAKind);
        assert_eq!(Ranking::from("33333"), Ranking::FiveOfAKind);

        assert_eq!(
            day7_fst_solve(&mut vec![
                Hand::new("32T3K", 765),
                Hand::new("T55J5", 684),
                Hand::new("KK677", 28),
                Hand::new("KTJJT", 220),
                Hand::new("QQQJA", 483),
            ]),
            6440
        );

        assert_eq!(Ranking::from_with_wildcards("32T3K"), Ranking::Pair);
        assert_eq!(Ranking::from_with_wildcards("KK677"), Ranking::TwoPair);
        assert_eq!(Ranking::from_with_wildcards("T55J5"), Ranking::FourOfAKind);
        assert_eq!(Ranking::from_with_wildcards("KTJJT"), Ranking::FourOfAKind);
        assert_eq!(Ranking::from_with_wildcards("QQQJA"), Ranking::FourOfAKind);

        assert_eq!(
            day7_snd_solve(&mut vec![
                Hand::new("32T3K", 765),
                Hand::new("T55J5", 684),
                Hand::new("KK677", 28),
                Hand::new("KTJJT", 220),
                Hand::new("QQQJA", 483),
            ]),
            5905
        );
    }

    #[test]
    fn ordering() {
        assert!(Ranking::Pair > Ranking::HighCard);
        assert!(Ranking::TwoPair > Ranking::Pair);
        assert!(Ranking::ThreeOfAKind > Ranking::TwoPair);
        assert!(Ranking::FullHouse > Ranking::ThreeOfAKind);
        assert!(Ranking::FourOfAKind > Ranking::FullHouse);
        assert!(Ranking::FiveOfAKind > Ranking::FourOfAKind);

        // part 1
        assert_eq!(compare("82T3A", "T2J3A"), Ordering::Less);
        assert_eq!(compare("77888", "88877"), Ordering::Less);
        assert_eq!(compare("8TT3A", "8TT3A"), Ordering::Equal);
        assert_eq!(compare("8TT3A", "T2J3A"), Ordering::Greater);
        assert_eq!(compare("33332", "2AAAA"), Ordering::Greater);
        assert_eq!(compare("77888", "77788"), Ordering::Greater);

        // part 2
        assert_eq!(compare2("82T3A", "T2J3A"), Ordering::Less);
        assert_eq!(compare2("77888", "88877"), Ordering::Less);
        assert_eq!(compare2("8TT3A", "8TT3A"), Ordering::Equal);
        assert_eq!(compare2("33332", "2AAAA"), Ordering::Greater);
        assert_eq!(compare2("77888", "77788"), Ordering::Greater);
        assert_eq!(compare2("JKKK2", "QQQQ2"), Ordering::Less);
        assert_eq!(compare2("JKKK2", "KKKK2"), Ordering::Less);
        assert_eq!(compare2("3AAAA", "3AJAA"), Ordering::Greater);
        assert_eq!(compare2("JJJJJ", "AAAAA"), Ordering::Less);

        assert_eq!(compare2("99A9A", "9T9J2"), Ordering::Greater);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day7_fst(), 246424613);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day7_snd(), 248256639);
    }
}
