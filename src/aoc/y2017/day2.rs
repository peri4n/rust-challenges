use std::fs;

const INPUT_FILE: &str = "src/aoc/y2017/day2.txt";

fn input() -> Vec<Vec<u32>> {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file: 2017/day2.txt");
    content
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
        .collect()
}

pub fn day2_fst() -> u32 {
    input()
        .iter()
        .map(|row| {
            let (min, max) = row.iter().fold((u32::MAX, u32::MIN), |(min, max), &val| {
                (min.min(val), max.max(val))
            });
            max - min
        })
        .sum()
}

pub fn day2_snd() -> u32 {
    input()
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
