use std::{collections::HashMap, fs, hash::{DefaultHasher, Hash, Hasher}};

const INPUT_FILE: &str = "src/aoc/y2017/day6.txt";

fn input() -> Vec<u32> {
    fs::read_to_string(INPUT_FILE).expect("Failed to read input file: 2017/day6.txt")
        .split('\t')
        .map(|n| n.trim().parse::<u32>().expect("Invalid number detected"))
        .collect()
}

pub fn day6_fst() -> usize {
    let setup = input();
    Banks::new(setup).into_iter().count() + 1
}

pub fn day6_snd() -> usize {
    let setup = input();
    let mut iter = Banks::new(setup).into_iter();

    // Consume iterator until it detects cycle
    while iter.next().is_some() {}

    // Return cycle length
    iter.cycle_length().unwrap() as usize
}

#[derive(Hash)]
struct Banks {
    banks: Vec<u32>,
}

impl Banks {
    pub fn new(banks: Vec<u32>) -> Self {
        Self { banks }
    }

    fn max_index(&self) -> usize {
        let mut max_index = 0;
        let mut current_max = u32::MIN;
        for (idx, value) in self.banks.iter().enumerate() {
            if *value > current_max {
                max_index = idx;
                current_max = *value;
            }
        }
        max_index
    }
}

impl IntoIterator for Banks {
    type Item = u64;

    type IntoIter= BanksIter;

    fn into_iter(self) -> Self::IntoIter {
        BanksIter {inner: self, seen_states: HashMap::new(), iteration: 0, cycle_length: None}
    }
}

struct BanksIter {
    inner: Banks,
    seen_states: HashMap<u64,u32>,
    iteration: u32,
    cycle_length: Option<u32>,
}

impl BanksIter {
    pub fn cycle_length(&self) -> Option<u32> {
        self.cycle_length
    }
}

impl Iterator for BanksIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // find max
        let max_index = self.inner.max_index();

        // get value at max
        let value = self.inner.banks[max_index];

        // set value to 0
        self.inner.banks[max_index] = 0;

        // spend all values
        let mut i = max_index;
        for _ in 0..value {
            i += 1;
            i = i % self.inner.banks.len();
            self.inner.banks[i] += 1;
        }

        // check if this state was already present
        let mut hasher = DefaultHasher::new();
        self.inner.banks.hash(&mut hasher);
        let hash = hasher.finish();
        
        if let Some(first_seen) = self.seen_states.get(&hash) {
            self.cycle_length = Some(self.iteration - first_seen);
            return None;
        }

        self.seen_states.insert(hash, self.iteration);
        self.iteration += 1;
        Some(hash)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples_fst() {
        let banks = Banks::new(vec![0,2,7,0]);
        assert_eq!(banks.into_iter().count(), 4);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day6_fst(), 5042);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day6_snd(), 1086);
    }

    #[test]
    fn examples_snd() {
        let setup = vec![0,2,7,0];
        let mut iter = Banks::new(setup).into_iter();
        while iter.next().is_some() {}
        assert_eq!(iter.cycle_length().unwrap(), 4);
    }
}
