use std::fs;

const INPUT_FILE: &str = "src/aoc/y2024/day7.txt";

pub fn day7_fst() -> i128 {
    fs::read_to_string(INPUT_FILE).expect("Unable to read file")
        .lines()
        .map(Equation::from_line)
        .filter(|eq| eq.is_valid())
        .map(|eq| eq.lhs)
        .sum()
}

#[derive(Debug)]
pub struct Equation {
    lhs: i128,
    rhs: Vec<i32>,
}

impl Equation {
    pub fn new(lhs: i128, rhs: Vec<i32>) -> Self {
        Self { lhs, rhs }
    }

    pub fn from_line(line: &str) -> Self {
        // find first ':'
        let colon_index = line.find(':').unwrap();
        let lhs = line[..colon_index]
            .parse::<i128>()
            .unwrap();
        let rest = line[colon_index + 1..]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        Self { lhs, rhs: rest }
    }

    pub fn is_valid(&self) -> bool {
        Self::validate_rec(self.lhs, self.rhs.clone().into_iter().rev().collect(), 0)
    }

    /// Recursively checks if any combination of operators '+','-','*','/' can be applied to the rhs so that it equls the lhs
    fn validate_rec(lhs: i128, mut rhs: Vec<i32>, intermediate: i128) -> bool {
        if let Some(first) = rhs.pop() {
            let first = first as i128;
            Self::validate_rec(lhs, rhs.clone(), intermediate + first)
                || Self::validate_rec(lhs, rhs.clone(), intermediate - first)
                || Self::validate_rec(lhs, rhs.clone(), intermediate * first)
                || Self::validate_rec(lhs, rhs.clone(), intermediate / first)
        } else {
            lhs == intermediate
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn cases() {
        assert!(Equation::new(190, vec![10, 19]).is_valid());
        assert!(Equation::new(3267, vec![81, 40, 27]).is_valid());
        assert!(Equation::new(292, vec![11, 6, 16, 20]).is_valid());

        assert!(!Equation::new(83, vec![17, 5]).is_valid());
        assert!(!Equation::new(156, vec![15, 6]).is_valid());
        assert!(!Equation::new(7290, vec![6, 8, 6, 15]).is_valid());
        assert!(!Equation::new(161011, vec![16, 10, 13]).is_valid());
        assert!(!Equation::new(192, vec![17, 8, 14]).is_valid());
        assert!(!Equation::new(21037, vec![9, 7, 18, 13]).is_valid());
    }

    fn solution_fst() {
        assert_eq!(day7_fst(), 0);
    }
}
