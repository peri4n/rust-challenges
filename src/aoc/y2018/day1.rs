use std::{collections::HashSet, fs};

const INPUT_FILE: &str = "src/aoc/y2018/day1.txt";

fn frequencies(infinite: bool) -> Box<dyn Iterator<Item = i32>> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");

    let values: Vec<i32> = content.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    if infinite {
        Box::new(values.into_iter().cycle())
    } else {
        Box::new(values.into_iter())
    }
}

pub fn day1_fst() -> i32 {
    frequencies(false).sum()
}

pub fn day1_snd() -> i32 {
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut frequency = 0;
    for f in frequencies(true) {
        frequency += f;

        if seen.contains(&frequency) {
            return frequency;
        }

        seen.insert(frequency);
    }
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 427);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 341);
    }
}
