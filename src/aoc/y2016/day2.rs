use std::fs;

use nom::{
    branch::alt, bytes::complete::tag, combinator::value, multi::many0, sequence::terminated,
    IResult,
};

const INPUT_FILE: &str = "src/aoc/y2016/day2.txt";

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Key {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Key {
    pub fn to(&self, direction: &Direction) -> Self {
        match self {
            Key::One => match direction {
                Direction::Right => Key::Two,
                Direction::Down => Key::Four,
                _ => Key::One,
            },
            Key::Two => match direction {
                Direction::Left => Key::One,
                Direction::Right => Key::Three,
                Direction::Down => Key::Five,
                _ => Key::Two,
            },
            Key::Three => match direction {
                Direction::Left => Key::Two,
                Direction::Down => Key::Six,
                _ => Key::Three,
            },
            Key::Four => match direction {
                Direction::Up => Key::One,
                Direction::Right => Key::Five,
                Direction::Down => Key::Seven,
                _ => Key::Four,
            },
            Key::Five => match direction {
                Direction::Up => Key::Two,
                Direction::Left => Key::Four,
                Direction::Right => Key::Six,
                Direction::Down => Key::Eight,
            },
            Key::Six => match direction {
                Direction::Up => Key::Three,
                Direction::Left => Key::Five,
                Direction::Down => Key::Nine,
                _ => Key::Six,
            },
            Key::Seven => match direction {
                Direction::Up => Key::Four,
                Direction::Right => Key::Eight,
                _ => Key::Seven,
            },
            Key::Eight => match direction {
                Direction::Up => Key::Five,
                Direction::Left => Key::Seven,
                Direction::Right => Key::Nine,
                _ => Key::Eight,
            },
            Key::Nine => match direction {
                Direction::Up => Key::Six,
                Direction::Left => Key::Eight,
                _ => Key::Nine,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HexKey {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl HexKey {
    pub fn char(&self) -> char {
        match self {
            HexKey::One => '1',
            HexKey::Two => '2',
            HexKey::Three => '3',
            HexKey::Four => '4',
            HexKey::Five => '5',
            HexKey::Six => '6',
            HexKey::Seven => '7',
            HexKey::Eight => '8',
            HexKey::Nine => '9',
            HexKey::A => 'A',
            HexKey::B => 'B',
            HexKey::C => 'C',
            HexKey::D => 'D',
        }
    }

    pub fn to(&self, direction: &Direction) -> Self {
        match self {
            HexKey::One => match direction {
                Direction::Down => HexKey::Three,
                _ => HexKey::One,
            },
            HexKey::Two => match direction {
                Direction::Right => HexKey::Three,
                Direction::Down => HexKey::Six,
                _ => HexKey::One,
            },
            HexKey::Three => match direction {
                Direction::Up => HexKey::One,
                Direction::Left => HexKey::Two,
                Direction::Right => HexKey::Four,
                Direction::Down => HexKey::Seven,
            },
            HexKey::Four => match direction {
                Direction::Left => HexKey::Three,
                Direction::Down => HexKey::Eight,
                _ => HexKey::Four,
            },
            HexKey::Five => match direction {
                Direction::Right => HexKey::Six,
                _ => HexKey::Five,
            },
            HexKey::Six => match direction {
                Direction::Up => HexKey::Two,
                Direction::Left => HexKey::Five,
                Direction::Right => HexKey::Seven,
                Direction::Down => HexKey::A,
            },
            HexKey::Seven => match direction {
                Direction::Up => HexKey::Three,
                Direction::Left => HexKey::Six,
                Direction::Right => HexKey::Eight,
                Direction::Down => HexKey::B,
            },
            HexKey::Eight => match direction {
                Direction::Up => HexKey::Four,
                Direction::Left => HexKey::Seven,
                Direction::Right => HexKey::Nine,
                Direction::Down => HexKey::C,
            },
            HexKey::Nine => match direction {
                Direction::Left => HexKey::Eight,
                _ => HexKey::Nine,
            },
            HexKey::A => match direction {
                Direction::Up => HexKey::Six,
                Direction::Right => HexKey::B,
                _ => HexKey::A,
            },
            HexKey::B => match direction {
                Direction::Up => HexKey::Seven,
                Direction::Left => HexKey::A,
                Direction::Right => HexKey::C,
                Direction::Down => HexKey::D,
            },
            HexKey::C => match direction {
                Direction::Up => HexKey::Eight,
                Direction::Left => HexKey::B,
                _ => HexKey::C,
            },
            HexKey::D => match direction {
                Direction::Up => HexKey::B,
                _ => HexKey::D,
            },
        }
    }
}

fn input() -> Vec<Vec<Direction>> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    parse_input(&content)
        .expect("Something went wrong while parsing")
        .1
}

fn parse_input(content: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    many0(terminated(parse_line, tag("\n")))(content)
}

fn parse_line(line: &str) -> IResult<&str, Vec<Direction>> {
    many0(alt((
        value(Direction::Up, tag("U")),
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
        value(Direction::Down, tag("D")),
    )))(line)
}

fn compute_code(start: Key, directions: Vec<Direction>) -> Key {
    let mut key = start;
    for direction in directions {
        key = key.to(&direction);
    }

    key
}

fn compute_hexcode(start: HexKey, directions: Vec<Direction>) -> HexKey {
    let mut key = start;
    for direction in directions {
        key = key.to(&direction);
    }

    key
}

pub fn day2_fst() -> u32 {
    let all_directions = input();

    let mut result = 0;
    let mut key = Key::Five;
    for directions in all_directions {
        result *= 10;
        key = compute_code(key.clone(), directions);
        result += key as u32;
    }

    result
}

pub fn day2_snd() -> String {
    let all_directions = input();

    let mut result = String::new();
    let mut key = HexKey::Five;
    for directions in all_directions {
        key = compute_hexcode(key.clone(), directions);
        result.push(key.char());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fst() {
        assert_eq!(
            compute_code(
                Key::Five,
                vec![Direction::Up, Direction::Left, Direction::Left]
            ),
            Key::One
        );
        assert_eq!(
            compute_code(
                Key::One,
                vec![
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down
                ]
            ),
            Key::Nine
        );
        assert_eq!(
            compute_code(
                Key::Nine,
                vec![
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left
                ]
            ),
            Key::Eight
        );
        assert_eq!(
            compute_code(
                Key::Eight,
                vec![
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down
                ]
            ),
            Key::Five
        );
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day2_fst(), 56855);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), String::from("B3C27"));
    }
}
