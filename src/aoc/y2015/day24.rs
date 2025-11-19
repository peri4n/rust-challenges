use itertools::Itertools;

const INPUT_FILE: &str = "src/aoc/y2015/day24.txt";

fn input() -> Vec<u64> {
    std::fs::read_to_string(INPUT_FILE)
        .expect("Could not read input file")
        .lines()
        .map(|line| line.parse::<u64>().expect("Could not parse line as u32"))
        .collect()
}

pub fn day24_fst() -> u64 {
    let weights = input();
    let total_weight: u64 = weights.iter().sum();
    let target_weight = total_weight / 3;

    for size in 1..=weights.len() {
        let mut min_qe = None;

        for combo in weights.iter().combinations(size) {
            let sum: u64 = combo.iter().copied().sum();
            if sum == target_weight {
                let qe: u64 = combo.iter().copied().product();
                match min_qe {
                    Some(current_min) if qe < current_min => min_qe = Some(qe),
                    None => min_qe = Some(qe),
                    _ => {}
                }
            }
        }

        if let Some(qe) = min_qe {
            return qe;
        }
    }

    0
}

pub fn day24_snd() -> u64 {
    let weights = input();
    let total_weight: u64 = weights.iter().sum();
    let target_weight = total_weight / 4;

    for size in 1..=weights.len() {
        let mut min_qe = None;

        for combo in weights.iter().combinations(size) {
            let sum: u64 = combo.iter().copied().sum();
            if sum == target_weight {
                let qe: u64 = combo.iter().copied().product();
                match min_qe {
                    Some(current_min) if qe < current_min => min_qe = Some(qe),
                    None => min_qe = Some(qe),
                    _ => {}
                }
            }
        }

        if let Some(qe) = min_qe {
            return qe;
        }
    }

    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day24_fst(), 10723906903);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day24_snd(), 74850409);
    }

}
