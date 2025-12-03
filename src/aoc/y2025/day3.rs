use std::fs;

const INPUT_FILE: &str = "src/aoc/y2025/day3.txt";

fn input() -> Vec<Vec<u8>> {
    fs::read_to_string(INPUT_FILE).expect("Failed to read input file")
        .lines()
        .map(|line| line.bytes().map(|b| b & 0b1111).collect())
        .collect()
}

pub fn day3_fst() -> u32 {
    let batteries = input();

    let mut sum = 0;
    for cells in batteries {
        sum += largest_subsequence(&cells, 2);
    }

    sum as u32
}

pub fn day3_snd() -> u128 {
    let batteries = input();

    let mut sum = 0;
    for cells in batteries {
        sum += largest_subsequence(&cells, 12);
    }

    sum
}

pub fn largest_subsequence(digits: &[u8], k: usize) -> u128 {

    if digits.len() < k {
        return 0;
    }

    let mut result: Vec<u8> = Vec::with_capacity(k);
    let mut to_pick = k;

    let mut i = 0;
    while to_pick > 0 {
        let remaining = digits.len() - i;
        let end = remaining - (to_pick - 1);

        // Find the maximum digit in digits[i .. i+end)
        let mut max_digit = 0u8;
        let mut max_pos = i;

        for pos in i..i + end {
            let d = digits[pos];
            if d > max_digit {
                max_digit = d;
                max_pos = pos;
                if d == 9 { break; }
            }
        }

        result.push(max_digit);
        to_pick -= 1;
        i = max_pos + 1;
    }

    // Convert to number
    result.iter().fold(0u128, |acc, &d| acc * 10 + d as u128)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day3_fst(), 17095);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day3_snd(), 168794698570517);
    }
}
