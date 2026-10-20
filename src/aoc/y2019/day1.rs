use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day1.txt");

pub struct Day1 {
    masses: Vec<i32>,
}

impl Solution for Day1 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let masses = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
        Day1 { masses }
    }

    fn fst(&self) -> i32 {
        self.masses.iter().copied().map(fuel1).sum()
    }

    fn snd(&self) -> i32 {
        self.masses.iter().copied().map(fuel2).sum()
    }
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
    fn examples_snd() {
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
    }

    #[test]
    fn solution() {
        let day = Day1::parse(INPUT);
        assert_eq!(day.fst(), 3349352);
        assert_eq!(day.snd(), 5021154);
    }
}
