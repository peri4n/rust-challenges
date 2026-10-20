use crate::aoc::Solution;

pub const INPUT: &str = "1321131112";

fn run_length_encoding(str: &str) -> String {
    if str.is_empty() {
        return "".to_owned();
    }

    let mut res = String::with_capacity(str.len());
    let characters: Vec<_> = str.bytes().collect();
    let mut occurrences = 1;

    for i in 1..characters.len() {
        if characters[i - 1] == characters[i] {
            occurrences += 1;
        } else {
            res.push_str(&occurrences.to_string());
            res.push(characters[i - 1] as char);
            occurrences = 1;
        }
    }

    res.push_str(&occurrences.to_string());
    res.push(characters[characters.len() - 1] as char);

    res
}

fn day10_helper(sequence: &str, iter: usize) -> String {
    let mut str = sequence.to_string();

    for _ in 0..iter {
        str = run_length_encoding(&str);
    }

    str
}

pub struct Day10 {
    sequence: String,
}

impl Solution for Day10 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day10 {
            sequence: input.to_owned(),
        }
    }

    fn fst(&self) -> usize {
        day10_helper(&self.sequence, 40).len()
    }

    fn snd(&self) -> usize {
        day10_helper(&self.sequence, 50).len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(run_length_encoding("1"), "11");
        assert_eq!(run_length_encoding("11"), "21");
        assert_eq!(run_length_encoding("21"), "1211");
        assert_eq!(run_length_encoding("1211"), "111221");
        assert_eq!(run_length_encoding("111221"), "312211");
    }

    #[test]
    fn solution() {
        let day = Day10::parse(INPUT);
        assert_eq!(day.fst(), 492982);
        assert_eq!(day.snd(), 6989950);
    }
}
