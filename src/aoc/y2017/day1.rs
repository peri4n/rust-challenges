use std::fs;

const INPUT_FILE: &str = "src/aoc/y2017/day1.txt";

fn input() -> Vec<u32> {
    let content = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    content.chars().filter_map(|c| c.to_digit(10)).collect()
}

pub fn day1_fst() -> u32 {
    let digits = input();
    let mut sum = 0;

    for i in 0..digits.len() {
        if digits[i] == digits[(i + 1) % digits.len()] {
            sum += digits[i];
        }
    }
    sum
}

pub fn day1_snd() -> u32 {
    let digits = input();
    let mut sum = 0;
    let step = digits.len() / 2;

    for i in 0..digits.len() {
        if digits[i] == digits[(i + step) % digits.len()] {
            sum += digits[i];
        }
    }
    sum
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 1102);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 1076);
    }
}
