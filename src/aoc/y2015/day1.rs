use std::fs;

const INPUT_FILE: &str = "src/aoc/y2015/day1.txt";

fn input() -> String {
    fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file")
}

pub fn day1_fst() -> i32 {
    let contents = input();

    let mut story = 0;
    for direction in contents.chars() {
        match direction {
            '(' => story += 1,
            ')' => story -= 1,
            _ => panic!("This should never happen"),
        }
    }
    story
}

pub fn day1_snd() -> usize {
    let contents = input();

    let mut story = 0;
    for (i, direction) in contents.chars().enumerate() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 280);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 1797);
    }
}
