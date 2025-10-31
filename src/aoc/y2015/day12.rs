use regex::Regex;
use serde_json::*;
use std::fs;

const INPUT_FILE: &str = "src/aoc/y2015/day12.txt";

pub fn day12_fst() -> i32 {
    let re = Regex::new(r#"-?[0-9]+"#).unwrap();

    fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .lines()
        .map(|line| add_numbers(line, &re))
        .sum()
}

fn add_numbers(str: &str, regex: &Regex) -> i32 {
    regex
        .find_iter(str)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum()
}

pub fn day12_snd() -> i64 {
    let input = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let v: Value = serde_json::from_str(&input).unwrap();
    prune(&v)
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

#[cfg(test)]
mod test {
    use super::{day12_fst, day12_snd};

    #[test]
    fn solution_fst() {
        assert_eq!(day12_fst(), 156366);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day12_snd(), 96852);
    }
}
