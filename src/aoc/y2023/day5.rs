use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, newline, u64};
use nom::multi::{many_till, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, terminated};

struct Problem {
    seeds: Vec<Seed>,
    mappings: Vec<Mapping>,
}

impl Problem {
    pub fn apply_all_mappings(&self, seed: Seed) -> u64 {
        let mut res = seed;
        for mapping in &self.mappings {
            res = mapping.map(res);
        }
        res
    }

    pub fn seeds(&self) -> Seeds<'_> {
        Seeds {
            problem: self,
            current_pair: 0,
            current_offset: 0,
        }
    }
}

struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    pub fn map(&self, number: u64) -> u64 {
        for rule in &self.rules {
            if let Some(n) = rule.map(number) {
                return n;
            }
        }
        number
    }
}

struct Rule {
    destination: u64,
    source: u64,
    length: u64,
}

impl Rule {
    pub fn map(&self, number: u64) -> Option<u64> {
        let source_start = self.source;
        let source_end = self.source + self.length;

        if source_start <= number && number < source_end {
            let offset = number - source_start;

            return Some(self.destination + offset);
        }

        None
    }
}

type Seed = u64;

fn parse_problem(text: &str) -> IResult<&str, Problem> {
    terminated(parse_seeds, newline)
        .and(separated_list1(newline, parse_mapping))
        .map(|(seeds, mappings)| Problem { seeds, mappings })
        .parse(text)
}

fn parse_seeds(line: &str) -> IResult<&str, Vec<Seed>> {
    delimited(tag("seeds: "), separated_list0(char(' '), u64), newline)(line)
}

fn parse_mapping(text: &str) -> IResult<&str, Mapping> {
    preceded(
        many_till(anychar, newline),
        terminated(separated_list1(newline, parse_rule), newline),
    )
    .map(|rules| Mapping { rules })
    .parse(text)
}

fn parse_rule(line: &str) -> IResult<&str, Rule> {
    separated_list1(char(' '), u64)
        .map(|numbers| Rule {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        })
        .parse(line)
}

pub fn day5_fst() -> u64 {
    let content = include_str!("../y2023/day5.txt");
    let problem = parse_problem(content).unwrap().1;

    problem
        .seeds
        .iter()
        .map(|&seed| problem.apply_all_mappings(seed))
        .min()
        .unwrap()
}

struct Seeds<'a> {
    problem: &'a Problem,
    current_pair: usize,
    current_offset: usize,
}

impl<'a> Iterator for Seeds<'a> {
    type Item = Seed;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pair > self.problem.seeds.len() / 2 {
            return None;
        }

        let pair_start = self.problem.seeds[self.current_pair];
        let pair_end = pair_start + self.problem.seeds[self.current_pair + 1];

        if pair_start + self.current_offset as u64 >= pair_end {
            self.current_pair += 2;
            self.current_offset = 0;
            return self.next();
        }

        self.current_offset += 1;
        Some(pair_start + self.current_offset as u64 - 1)
    }
}

pub fn day5_snd() -> u64 {
    let content = include_str!("../y2023/day5.txt");
    let problem = parse_problem(content).unwrap().1;

    problem
        .seeds()
        .map(|seed| problem.apply_all_mappings(seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let rule = Rule {
            destination: 50,
            source: 98,
            length: 2,
        };
        assert_eq!(rule.map(97), None);
        assert_eq!(rule.map(98), Some(50));
        assert_eq!(rule.map(99), Some(51));

        let mapping = Mapping {
            rules: vec![
                Rule {
                    destination: 50,
                    source: 98,
                    length: 2,
                },
                Rule {
                    destination: 52,
                    source: 50,
                    length: 48,
                },
            ],
        };

        assert_eq!(mapping.map(0), 0);
        assert_eq!(mapping.map(50), 52);
        assert_eq!(mapping.map(96), 98);
        assert_eq!(mapping.map(97), 99);
        assert_eq!(mapping.map(98), 50);
        assert_eq!(mapping.map(99), 51);
    }

    #[test]
    fn seed_iterator() {
        let problem = Problem {
            seeds: vec![79, 14, 55, 13],
            mappings: vec![],
        };
        let mut solution = (79..=92).collect::<Vec<_>>();
        solution.extend((55..=67).collect::<Vec<_>>());

        assert_eq!(problem.seeds().collect::<Vec<_>>(), solution);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day5_fst(), 389056265);
    }

    #[test]
    #[ignore = "To slow"]
    fn solution_snd() {
        assert_eq!(day5_snd(), 137516820);
    }
}
