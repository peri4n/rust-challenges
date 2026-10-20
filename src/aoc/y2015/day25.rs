use crate::aoc::Solution;

pub const INPUT: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 3010, column 3019.";

pub struct Day25 {
    target_row: u64,
    target_col: u64,
}

impl Solution for Day25 {
    type Fst = u64;
    type Snd = ();

    fn parse(input: &str) -> Self {
        let mut numbers = input
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap());
        Day25 {
            target_row: numbers.next().unwrap(),
            target_col: numbers.next().unwrap(),
        }
    }

    fn fst(&self) -> u64 {
        let mut code = 20151125u64;
        let target_row = self.target_row;
        let target_col = self.target_col;

        let mut row = 1;
        let mut column = 1;

        while row != target_row || column != target_col {
            code = (code * 252533) % 33554393;

            if row == 1 {
                row = column + 1;
                column = 1;
            } else {
                row -= 1;
                column += 1;
            }
        }

        code
    }

    fn snd(&self) {
        // Day 25 has no second part
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day25::parse(INPUT);
        assert_eq!(day.fst(), 8997277);
    }
}
