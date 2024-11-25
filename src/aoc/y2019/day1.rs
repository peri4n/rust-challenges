use std::fs;

const INPUT_FILE: &str = "src/aoc/y2019/day1.txt";

pub fn day1_fst() -> i32 {
    fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(fuel1)
        .sum()
}

pub fn day1_snd() -> i32 {
    fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(fuel2)
        .sum()
}
fn fuel1(mass: i32) -> i32 {
    0.max(mass / 3 - 2)
}

fn fuel2(mass: i32) -> i32 {
    let f = fuel1(mass);

    if f > 0 {
        return f + fuel2(f);
    }

    f
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_fst() {
        assert_eq!(fuel1(14), 2);
        assert_eq!(fuel1(12), 2);
        assert_eq!(fuel1(1969), 654);
        assert_eq!(fuel1(100756), 33583);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day1_fst(), 3349352);
    }

    #[test]
    fn examples_snd() {
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day1_snd(), 5021154);
    }
}
