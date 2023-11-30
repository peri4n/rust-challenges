use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "src/aoc/y2015/day8.txt";

fn input() -> Vec<String> {
    fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

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

pub fn day8_fst() -> i32 {
    input().into_iter().fold(0, |extra_chars, line| {
        let (raw_len, unescaped_len) = count_unescaped_lenghts(&line);
        extra_chars + (raw_len - unescaped_len)
    })
}

pub fn day8_snd() -> i32 {
    input().into_iter().fold(0, |extra_chars, line| {
        let (raw_len, escaped_len) = count_escaped_lenghts(&line);
        extra_chars + (escaped_len - raw_len)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day8_fst(), 1333);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day8_snd(), 2046);
    }
}
