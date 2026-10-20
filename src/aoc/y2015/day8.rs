use regex::Regex;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day8.txt");

fn count_unescaped_lenghts(str: &str) -> (i32, i32) {
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();

    let raw_len = str.len() as i32;
    let (escaped_count, escaped_size) = re.find_iter(str).fold((0, 0), |(count, size), m| {
        (count + 1, size + m.end() - m.start())
    });

    (raw_len, raw_len - 2 - escaped_size as i32 + escaped_count)
}

fn count_escaped_lenghts(str: &str) -> (i32, i32) {
    let re = Regex::new(r#"[\\"]"#).unwrap();

    let raw_len = str.len() as i32;
    let escaped_count = re.find_iter(str).count();

    (raw_len, raw_len + 2 + escaped_count as i32)
}

pub struct Day8 {
    lines: Vec<String>,
}

impl Solution for Day8 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        Day8 {
            lines: input.lines().map(|l| l.to_owned()).collect(),
        }
    }

    fn fst(&self) -> i32 {
        self.lines.iter().fold(0, |extra_chars, line| {
            let (raw_len, unescaped_len) = count_unescaped_lenghts(line);
            extra_chars + (raw_len - unescaped_len)
        })
    }

    fn snd(&self) -> i32 {
        self.lines.iter().fold(0, |extra_chars, line| {
            let (raw_len, escaped_len) = count_escaped_lenghts(line);
            extra_chars + (escaped_len - raw_len)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day8::parse(INPUT);
        assert_eq!(day.fst(), 1333);
        assert_eq!(day.snd(), 2046);
    }
}
