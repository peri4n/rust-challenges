use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2017/day10.txt";

fn lengths() -> Vec<usize> {
    std::fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

fn lengths2() -> Vec<usize> {
    let mut lengths: Vec<usize> = std::fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    lengths
}

#[derive(Debug)]
struct Knots {
    numbers: [u8; 256],
    position: usize,
    skip: usize,
}

impl Knots {
    pub fn new() -> Self {
        Self {
            numbers: std::array::from_fn(|i| i as u8),
            position: 0,
            skip: 0,
        }
    }

    pub fn tie(&mut self, length: usize) {
        Self::reverse_circular(&mut self.numbers, self.position, length);

        self.position = (self.position + self.skip + length) % 256;
        self.skip = (self.skip + 1) % 256;
    }

    pub fn result(&self) -> usize {
        self.numbers[0] as usize * self.numbers[1] as usize
    }

    pub fn result2(&self) -> String {
        self.numbers.chunks(16)
            .map(|c| c.iter().fold(0, |acc, e| acc ^ *e))
            .map(|c| format!("{:02x}", c))
            .join("")
    }

    fn reverse_circular(arr: &mut [u8; 256], start: usize, len: usize) {
        for i in 0..len / 2 {
            let a = (start + i) % 256;
            let b = (start + len - 1 - i) % 256;
            arr.swap(a, b);
        }
    }
}

pub fn day10_fst() -> usize {
    let mut knots = Knots::new();

    for length in lengths() {
        knots.tie(length);
    }
    knots.result()
}

pub fn day10_snd() -> String {
    let mut knots = Knots::new();

    let lengths = lengths2();
    for _ in 0..64 {
        for &length in &lengths {
            knots.tie(length);
        }
    }
    knots.result2()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day10_fst(), 9656);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day10_snd(), String::from("20b7b54c92bf73cf3e5631458a715149"));
    }
}
