use std::{fs, u32};

const INPUT_FILE: &str = "src/aoc/y2017/day2.txt";

fn input() -> Vec<Vec<u32>> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    content.lines()
        .map(|line| line.split('\t')
            .map(|chunk| chunk.trim().parse::<u32>().unwrap())
            .collect())
        .collect()
}

pub fn day2_fst() -> u32 {
    input().iter().map(|row| row.iter().max().unwrap_or(&u32::MIN) - row.iter().min().unwrap_or(&u32::MAX)).sum()
}

pub fn day2_snd() -> u32 {
    input().iter().map(|row| {
        for (i, &x) in row.iter().enumerate() {
            for &y in &row[i + 1..] {
                if x % y == 0 {
                    return x / y;
                } else if y % x == 0 {
                    return y / x;
                }
            }
        }
        0
    }).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day2_fst(), 51833);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day2_snd(), 288);
    }
}
