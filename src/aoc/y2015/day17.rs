use std::fs;

pub fn day17_fst() -> i32 {
    let buckets = fs::read_to_string("src/aoc/y2015/day17.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
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

    // the bottom-right corner will have our answer.
    dp[nr_of_buckets - 1][sum as usize]
}

pub fn day17_snd() -> i32 {
    let mut buckets = fs::read_to_string("src/aoc/y2015/day17.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
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
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17_fst() {
        assert_eq!(day17_fst(), 1304);
    }

    #[test]
    fn test_day17_snd() {
        assert_eq!(day17_snd(), 18);
    }
}
