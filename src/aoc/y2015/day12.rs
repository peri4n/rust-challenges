use regex::Regex;
use serde_json::*;

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day12.txt");

fn add_numbers(str: &str, regex: &Regex) -> i32 {
    regex
        .find_iter(str)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum()
}

fn prune(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(prune).sum(),
        Value::Object(o) => {
            let contains_red = o.values().any(|v| *v == Value::String("red".to_string()));

            if !contains_red {
                return o.values().map(prune).sum();
            }
            0
        }
    }
}

pub struct Day12 {
    line_sums: Vec<i32>,
    document: Value,
}

impl Solution for Day12 {
    type Fst = i32;
    type Snd = i64;

    fn parse(input: &str) -> Self {
        let re = Regex::new(r#"-?[0-9]+"#).unwrap();

        let line_sums = input.lines().map(|line| add_numbers(line, &re)).collect();
        let document = serde_json::from_str(input).unwrap();
        Day12 { line_sums, document }
    }

    fn fst(&self) -> i32 {
        self.line_sums.iter().sum()
    }

    fn snd(&self) -> i64 {
        prune(&self.document)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day12::parse(INPUT);
        assert_eq!(day.fst(), 156366);
        assert_eq!(day.snd(), 96852);
    }
}
