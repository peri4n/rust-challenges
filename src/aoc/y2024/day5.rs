use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2024/day5.txt";

pub fn day5_fst() -> i32 {
    let (dependency_lines, update_lines): (Vec<_>, Vec<_>) = std::fs::read_to_string(INPUT_FILE)
        .expect("Unable to read input file")
        .lines()
        .map(|line| String::from(line))
        .filter(|line| !line.is_empty())
        .partition(|line| line.contains('|'));

    let dependencies = create_dependencies(dependency_lines);
    let updates = create_updates(update_lines);

    updates
        .iter()
        .filter(|&update| is_in_topological_order(update, &dependencies))
        .map(|update| update[(update.len() as f32 / 2.0).floor() as usize])
        .sum()
}

pub fn day5_snd() -> i32 {
    let (dependency_lines, update_lines): (Vec<_>, Vec<_>) = std::fs::read_to_string(INPUT_FILE)
        .expect("Unable to read input file")
        .lines()
        .map(|line| String::from(line))
        .filter(|line| !line.is_empty())
        .partition(|line| line.contains('|'));

    let dependencies = create_dependencies(dependency_lines);
    let updates = create_updates(update_lines);

    updates
        .iter()
        .filter(|&update| !is_in_topological_order(update, &dependencies))
        .map(|update| correct_order(update.clone(), &dependencies))
        .map(|update| update[(update.len() as f32 / 2.0).floor() as usize])
        .sum()
}

fn is_in_topological_order(update: &Vec<i32>, dependencies: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, number) in update.iter().enumerate() {
        let suffix = &update[i + 1..];
        if let Some(deps) = dependencies.get(&number) {
            if suffix.iter().any(|n| deps.contains(n)) {
                return false;
            }
        }
    }

    true
}

fn correct_order(mut update: Vec<i32>, dependencies: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    update.sort_by(|&a, &b| {
        if let Some(deps_a) = dependencies.get(&a) {
            if deps_a.contains(&b) {
                return std::cmp::Ordering::Less;
            }
        }

        if let Some(deps_b) = dependencies.get(&b) {
            if deps_b.contains(&a) {
                return std::cmp::Ordering::Greater;
            }
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
    fn solution_fst() {
        assert_eq!(day5_fst(), 7307);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day5_snd(), 4713);
    }
}
