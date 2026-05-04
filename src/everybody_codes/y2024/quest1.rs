use std::fs;

const PART1_FILE: &str = "src/everybody_codes/y2024/quest1_1.txt";
const PART2_FILE: &str = "src/everybody_codes/y2024/quest1_2.txt";
const PART3_FILE: &str = "src/everybody_codes/y2024/quest1_3.txt";

pub fn quest1_part1() -> usize {
    let content =
        fs::read_to_string(PART1_FILE).expect("Should have been able to read the input file");

    content
        .as_bytes()
        .iter()
        .map(char_to_potion)
        .sum::<usize>()
}

pub fn quest1_part2() -> usize {
    let content =
        fs::read_to_string(PART2_FILE).expect("Should have been able to read the input file");

    content
        .as_bytes()
        .chunks(2)
        .map(|w| match w {
            [b'x', b'x'] => 0,
            [b'x', m] => char_to_potion(m),
            [m, b'x'] => char_to_potion(m),
            [m1, m2] => char_to_potion(m1) + char_to_potion(m2) + 2,
            _ => panic!("Unknown monster"),
        })
        .sum::<usize>()
}

pub fn quest1_part3() -> usize {
    let content =
        fs::read_to_string(PART3_FILE).expect("Should have been able to read the input file");

    content
        .as_bytes()
        .chunks(3)
        .map(|w| match w {
            [b'x', b'x', b'x'] => 0,
            [m, b'x', b'x'] => char_to_potion(m),
            [b'x', m, b'x'] => char_to_potion(m),
            [b'x', b'x', m] => char_to_potion(m),
            [m1, m2, b'x'] => char_to_potion(m1) + char_to_potion(m2) + 2,
            [b'x', m1, m2] => char_to_potion(m1) + char_to_potion(m2) + 2,
            [m1, b'x', m2] => char_to_potion(m1) + char_to_potion(m2) + 2,
            [m1, m2, m3] => char_to_potion(m1) + char_to_potion(m2) + char_to_potion(m3) + 6,
            _ => panic!("Unknown monster"),
        })
        .sum::<usize>()
}

fn char_to_potion(monster: &u8) -> usize {
    match *monster {
        b'A' => 0,
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        m => panic!("Unknown monster: {}", m),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest1_part1_solution() {
        assert_eq!(quest1_part1(), 1404);
    }

    #[test]
    fn quest1_part2_solution() {
        assert_eq!(quest1_part2(), 5237);
    }

    #[test]
    fn quest1_part3_solution() {
        assert_eq!(quest1_part3(), 5237);
    }
}
