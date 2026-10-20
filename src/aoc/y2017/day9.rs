use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day9.txt");

pub struct Day9 {
    stream: String,
}

impl Solution for Day9 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day9 {
            stream: input.to_string(),
        }
    }

    fn fst(&self) -> usize {
        let mut score = 0;
        let mut depth = 0;
        let mut in_garbage = false;
        let mut skip_next = false;

        for c in self.stream.chars() {
            if skip_next {
                skip_next = false;
                continue;
            }

            match c {
                '!' if in_garbage => skip_next = true,
                '<' if !in_garbage => in_garbage = true,
                '>' if in_garbage => in_garbage = false,
                '{' if !in_garbage => depth += 1,
                '}' if !in_garbage => {
                    score += depth;
                    depth -= 1;
                }
                _ => {}
            }
        }

        score
    }

    fn snd(&self) -> usize {
        let mut garbage_count = 0;
        let mut in_garbage = false;
        let mut skip_next = false;

        for c in self.stream.chars() {
            if skip_next {
                skip_next = false;
                continue;
            }

            match c {
                '!' if in_garbage => skip_next = true,
                '<' if !in_garbage => in_garbage = true,
                '>' if in_garbage => in_garbage = false,
                _ if in_garbage => garbage_count += 1,
                _ => {}
            }
        }

        garbage_count
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day9::parse(INPUT);
        assert_eq!(day.fst(), 14204);
        assert_eq!(day.snd(), 6622);
    }
}
