use nom::character::complete::i32;
use nom::{IResult, bytes::complete::tag, multi::separated_list1};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day2.txt");

pub struct Day2 {
    reports: Vec<Report>,
}

impl Solution for Day2 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        let reports = parse_reports(input).expect("Could not parse input").1;
        Day2 { reports }
    }

    fn fst(&self) -> usize {
        self.reports.iter().filter(|r| r.safe()).count()
    }

    fn snd(&self) -> usize {
        self.reports.iter().filter(|r| r.damp_safe()).count()
    }
}

fn parse_reports(content: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(tag("\n"), parse_report)(content)
}

fn parse_report(content: &str) -> IResult<&str, Report> {
    let (content, ls) = separated_list1(tag(" "), i32)(content)?;
    Ok((content, Report::new(ls)))
}

#[derive(Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    pub fn safe(&self) -> bool {
        if self.levels[0] == self.levels[1] {
            return false;
        }

        let increasing = self.levels[1] > self.levels[0];

        if increasing {
            for i in 0..self.levels.len() - 1 {
                let diff_to_next = self.levels[i + 1] - self.levels[i];
                if !(1..=3).contains(&diff_to_next) {
                    return false;
                }
            }
        } else {
            for i in 0..self.levels.len() - 1 {
                let diff_to_next = self.levels[i + 1] - self.levels[i];
                if !(-3..=-1).contains(&diff_to_next) {
                    return false;
                }
            }
        }
        true
    }

    fn drop_level(&mut self, idx: usize) {
        self.levels.remove(idx);
    }

    pub fn damp_safe(&self) -> bool {
        for i in 0..self.levels.len() {
            let mut copy = self.clone();
            copy.drop_level(i);

            if copy.safe() {
                return true;
            }
        }
        self.safe()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_fst() {
        assert!(Report::new(vec![7, 6, 4, 2, 1]).safe());
        assert!(Report::new(vec![1, 3, 6, 7, 9]).safe());

        assert!(!Report::new(vec![1, 2, 7, 8, 9]).safe());
        assert!(!Report::new(vec![9, 7, 6, 2, 1]).safe());
        assert!(!Report::new(vec![1, 3, 2, 4, 5]).safe());
        assert!(!Report::new(vec![8, 6, 4, 4, 1]).safe());
    }

    #[test]
    fn examples_snd() {
        assert!(Report::new(vec![7, 6, 4, 2, 1]).damp_safe());
        assert!(Report::new(vec![1, 3, 2, 4, 5]).damp_safe());
        assert!(Report::new(vec![8, 6, 4, 4, 1]).damp_safe());
        assert!(Report::new(vec![1, 3, 6, 7, 9]).damp_safe());

        assert!(!Report::new(vec![1, 2, 7, 8, 9]).damp_safe());
        assert!(!Report::new(vec![9, 7, 6, 2, 1]).damp_safe());
    }

    #[test]
    fn solution() {
        let day = Day2::parse(INPUT);
        assert_eq!(day.fst(), 686);
        assert_eq!(day.snd(), 717);
    }
}
