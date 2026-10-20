use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day2.txt");

pub struct Day2 {
    rows: Vec<Vec<u32>>,
}

impl Solution for Day2 {
    type Fst = u32;
    type Snd = u32;

    fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.split('\t')
                    .map(|chunk| {
                        chunk
                            .trim()
                            .parse::<u32>()
                            .expect("Invalid number in input")
                    })
                    .collect()
            })
            .collect();
        Day2 { rows }
    }

    fn fst(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| {
                let (min, max) = row.iter().fold((u32::MAX, u32::MIN), |(min, max), &val| {
                    (min.min(val), max.max(val))
                });
                max - min
            })
            .sum()
    }

    fn snd(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| {
                for (i, &x) in row.iter().enumerate() {
                    for &y in &row[i + 1..] {
                        let (larger, smaller) = if x > y { (x, y) } else { (y, x) };
                        if larger % smaller == 0 {
                            return larger / smaller;
                        }
                    }
                }
                panic!("No evenly divisible pair found in row")
            })
            .sum()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day2::parse(INPUT);
        assert_eq!(day.fst(), 51833);
        assert_eq!(day.snd(), 288);
    }
}
