use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day17.txt");

pub struct Day17 {
    buckets: Vec<i32>,
}

impl Solution for Day17 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let buckets = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Day17 { buckets }
    }

    fn fst(&self) -> i32 {
        let buckets = &self.buckets;
        let nr_of_buckets = buckets.len();

        let sum = 150;
        let mut dp = vec![vec![0; sum as usize + 1]; buckets.len()];

        // populate the sum=0 columns, as we will always have an empty set for zero sum
        for row in &mut dp {
            row[0] = 1;
        }

        // with only one number, we can form a subset only when the required sum is equal to its value
        for j in 1..sum + 1 {
            dp[0][j as usize] = if buckets[0] == j { 1 } else { 0 };
        }

        // process all subsets for all sums
        for i in 1..nr_of_buckets {
            for j in 1..sum + 1 {
                // exclude the number
                dp[i][j as usize] = dp[i - 1][j as usize];
                // include the number, if it does not exceed the sum
                if j >= buckets[i] {
                    dp[i][j as usize] += dp[i - 1][(j - buckets[i]) as usize];
                }
            }
        }

        // the bottom-right corner will have our answer.
        dp[nr_of_buckets - 1][sum as usize]
    }

    fn snd(&self) -> i32 {
        let mut buckets = self.buckets.clone();
        buckets.sort();
        let nr_of_buckets = buckets.len();

        let sum = 150;
        let mut dp = vec![vec![0; sum as usize + 1]; buckets.len()];

        // populate the sum=0 columns, as we will always have an empty set for zero sum
        for i in 0..buckets.len() {
            dp[i][0] = 1;
        }

        // with only one number, we can form a subset only when the required sum is equal to its value
        for j in 1..sum + 1 {
            dp[0][j as usize] = if buckets[0] == j { 1 } else { 0 };
        }

        // process all subsets for all sums
        for i in 1..nr_of_buckets {
            for j in 1..sum + 1 {
                // exclude the number
                dp[i][j as usize] = dp[i - 1][j as usize];
                // include the number, if it does not exceed the sum
                if j >= buckets[i] {
                    dp[i][j as usize] += dp[i - 1][(j - buckets[i]) as usize];
                }
            }
        }

        for i in 1..nr_of_buckets {
            if dp[i][sum as usize] > 1 {
                return dp[i][sum as usize];
            }
        }

        // does not happen
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day17::parse(INPUT);
        assert_eq!(day.fst(), 1304);
        assert_eq!(day.snd(), 18);
    }
}
