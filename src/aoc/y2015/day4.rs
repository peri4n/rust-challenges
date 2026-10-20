use md5::compute;

use crate::aoc::Solution;

pub const INPUT: &str = "iwrupvqb";

pub struct Day4 {
    key: String,
}

impl Solution for Day4 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day4 { key: input.to_owned() }
    }

    fn fst(&self) -> usize {
        let mut i = 0;
        loop {
            let hash = compute([self.key.as_str(), &i.to_string()].concat());
            if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4) == 0 {
                return i;
            }

            i += 1;
        }
    }

    fn snd(&self) -> usize {
        let mut i = 0;
        loop {
            let hash = compute([self.key.as_str(), &i.to_string()].concat());
            if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                return i;
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "To slow"]
    fn solution() {
        let day = Day4::parse(INPUT);
        assert_eq!(day.fst(), 346386);
        assert_eq!(day.snd(), 9958218);
    }
}
