use std::collections::{HashMap, HashSet};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day5.txt");

pub struct Day5 {
    dependencies: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

impl Solution for Day5 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let (dependency_lines, update_lines): (Vec<_>, Vec<_>) = input
            .lines()
            .map(String::from)
            .filter(|line| !line.is_empty())
            .partition(|line| line.contains('|'));

        Day5 {
            dependencies: create_dependencies(dependency_lines),
            updates: create_updates(update_lines),
        }
    }

    fn fst(&self) -> i32 {
        self.updates
            .iter()
            .filter(|&update| is_in_topological_order(update, &self.dependencies))
            .map(|update| update[(update.len() as f32 / 2.0).floor() as usize])
            .sum()
    }

    fn snd(&self) -> i32 {
        self.updates
            .iter()
            .filter(|&update| !is_in_topological_order(update, &self.dependencies))
            .map(|update| correct_order(update.clone(), &self.dependencies))
            .map(|update| update[(update.len() as f32 / 2.0).floor() as usize])
            .sum()
    }
}

fn is_in_topological_order(update: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, number) in update.iter().enumerate() {
        let suffix = &update[i + 1..];
        if let Some(deps) = dependencies.get(number)
            && suffix.iter().any(|n| deps.contains(n))
        {
            return false;
        }
    }

    true
}

fn correct_order(mut update: Vec<i32>, dependencies: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    update.sort_by(|&a, &b| {
        if let Some(deps_a) = dependencies.get(&a)
            && deps_a.contains(&b)
        {
            return std::cmp::Ordering::Less;
        }

        if let Some(deps_b) = dependencies.get(&b)
            && deps_b.contains(&a)
        {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal // If no rules apply, leave order unchanged
    });

    update
}

fn create_dependencies(lines: Vec<String>) -> HashMap<i32, HashSet<i32>> {
    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in lines {
        // Split the line into two parts by the first '|'
        let mut parts = line.split('|');
        let dependency = parts.next().unwrap().parse::<i32>().unwrap();
        let node = parts.next().unwrap().parse::<i32>().unwrap();

        // Create a new entry in the HashMap for the 'from' part
        dependencies
            .entry(node)
            .and_modify(|s| {
                s.insert(dependency);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(dependency);
                set
            });
    }

    dependencies
}

fn create_updates(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut updates = vec![];

    for line in lines {
        let parts = line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        updates.push(parts)
    }

    updates
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day5::parse(INPUT);
        assert_eq!(day.fst(), 7307);
        assert_eq!(day.snd(), 4713);
    }
}
