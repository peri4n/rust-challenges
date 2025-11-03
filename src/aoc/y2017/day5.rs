use std::fs;

const INPUT_FILE: &str = "src/aoc/y2017/day5.txt";

pub fn day5_fst() -> usize {
    let input = input();
    let cpu = CPU::new(input, false);

    cpu.count() + 1
}

pub fn day5_snd() -> usize {
    let input = input();
    let cpu = CPU::new(input, true);

    cpu.count() + 1
}

fn input() -> Vec<i32> {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file: 2017/day5.txt");
    content
        .lines()
        .map(|line| line.trim().parse::<i32>().expect("Invalid number in input"))
        .collect()
}

#[derive(Debug)]
struct CPU {
    current_pos: usize,
    instructions: Vec<i32>,
    part2: bool,
}

impl CPU {
    pub fn new(instructions: Vec<i32>, part2: bool) -> Self {
        Self {
            current_pos: 0,
            instructions,
            part2,
        }
    }
}

impl Iterator for CPU {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_position = self.current_pos as i32 + self.instructions[self.current_pos];

        if new_position < 0 || new_position >= self.instructions.len() as i32 {
            return None;
        }

        let old_position = self.current_pos;
        // Different increment logic based on part
        if self.part2 && self.instructions[old_position] >= 3 {
            self.instructions[old_position] -= 1;
        } else {
            self.instructions[old_position] += 1;
        }
        self.current_pos = new_position as usize;
        Some(self.instructions[self.current_pos])
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples_fst() {
        let cpu = CPU::new(vec![0, 3, 0, 1, -3], false);
        assert_eq!(cpu.count(), 4);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day5_fst(), 381680);
    }
}
    #[test]
    fn examples_snd() {
        let cpu = CPU::new(vec![0, 3, 0, 1, -3], true);
        assert_eq!(cpu.count(), 9);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day5_snd(), 29717847);
    }
