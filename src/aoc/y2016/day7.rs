use std::collections::HashSet;

use nom::{
    IResult,
    character::complete::{alphanumeric1, char},
    multi::many1,
    sequence::{preceded, terminated, tuple},
};

use crate::aoc::Solution;

#[derive(Debug)]
struct Ipv7 {
    supernet_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl Ipv7 {
    pub fn new(flanks: Vec<String>, hypernet_sequences: Vec<String>) -> Self {
        Self {
            supernet_sequences: flanks,
            hypernet_sequences,
        }
    }

    pub fn supports_tls(&self) -> bool {
        self.supernet_sequences.iter().any(|f| has_abba(f))
            && self.hypernet_sequences.iter().all(|f| !has_abba(f))
    }

    pub fn supports_ssl(&self) -> bool {
        let supernet_triplets = extract_triplets(&self.supernet_sequences);
        let hypernet_triplets = extract_triplets(&self.hypernet_sequences);

        for triplet in supernet_triplets.into_iter() {
            let comp = complement(&triplet);
            if hypernet_triplets.contains(&comp) {
                return true;
            }
        }
        false
    }
}

fn complement(str: &str) -> String {
    let bytes = str.as_bytes();
    let mut res = String::with_capacity(str.len());
    res.push(bytes[1] as char);
    res.push(bytes[0] as char);
    res.push(bytes[1] as char);

    res
}

fn has_abba(str: &str) -> bool {
    let chars = str.as_bytes();

    for i in 0..chars.len() - 3 {
        if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }

    false
}

fn extract_triplets(sequences: &Vec<String>) -> HashSet<String> {
    let mut all_triplets = HashSet::new();

    for supernet in sequences {
        let chars = supernet.as_bytes();
        for i in 0..chars.len() - 2 {
            if chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
                all_triplets
                    .insert(String::from_utf8(vec![chars[i], chars[i + 1], chars[i + 2]]).unwrap());
            }
        }
    }
    all_triplets
}

pub const INPUT: &str = include_str!("day7.txt");

fn parse_input(content: &str) -> IResult<&str, Vec<Ipv7>> {
    many1(terminated(parse_ip, char('\n')))(content)
}

fn parse_ip(content: &str) -> IResult<&str, Ipv7> {
    let (content, pairs) = many1(tuple((
        alphanumeric1,
        terminated(preceded(char('['), alphanumeric1), char(']')),
    )))(content)?;
    let (content, last) = alphanumeric1(content)?;

    let mut flanks = vec![];
    let mut hypernet_sequences = vec![];

    for p in pairs {
        flanks.push(p.0.to_owned());
        hypernet_sequences.push(p.1.to_owned());
    }
    flanks.push(last.to_owned());

    Ok((content, Ipv7::new(flanks, hypernet_sequences)))
}

pub struct Day7 {
    ips: Vec<Ipv7>,
}

impl Solution for Day7 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let ips = parse_input(input)
            .expect("Something went wrong while parsing")
            .1;
        Day7 { ips }
    }

    fn fst(&self) -> usize {
        self.ips.iter().filter(|ip| ip.supports_tls()).count()
    }

    fn snd(&self) -> usize {
        self.ips.iter().filter(|ip| ip.supports_ssl()).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day7::parse(INPUT);
        assert_eq!(day.fst(), 110);
        assert_eq!(day.snd(), 242);
    }
}
