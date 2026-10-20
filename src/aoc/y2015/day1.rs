use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

pub struct Day1 {
    directions: Vec<char>,
}

impl Solution for Day1 {
    type Fst = i32;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day1 {
            directions: input.chars().collect(),
        }
    }

    fn fst(&self) -> i32 {
        let mut story = 0;
        for direction in &self.directions {
            match direction {
                '(' => story += 1,
                ')' => story -= 1,
                _ => panic!("This should never happen"),
            }
        }
        story
    }

    fn snd(&self) -> usize {
        let mut story = 0;
        for (i, direction) in self.directions.iter().enumerate() {
            match direction {
                '(' => story += 1,
                ')' => story -= 1,
                _ => panic!("This should never happen"),
            }

            if story == -1 {
                return i + 1; // 1-indexed
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 280);
        assert_eq!(day.snd(), 1797);
    }
}
