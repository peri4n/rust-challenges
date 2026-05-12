use std::fs;

const PART1_FILE: &str = "src/everybody_codes/y2024/quest2_1.txt";
const PART2_FILE: &str = "src/everybody_codes/y2024/quest2_2.txt";

pub fn quest2_part1() -> usize {
    let content =
        fs::read_to_string(PART1_FILE).expect("Should have been able to read the input file");

    let (runic_words, scripture) = parse_runic_words(&content, false);

    let mut count = 0;
    for word in &runic_words {
        for i in 0..scripture.len() {
            if scripture[i..].starts_with(word) {
                count += 1;
            }
        }
    }
    count
}

pub fn quest2_part2() -> usize {
    let content =
        fs::read_to_string(PART2_FILE).expect("Should have been able to read the input file");

    let (runic_words, scripture) = parse_runic_words(&content, true);

    let mut runes = vec![0; scripture.len()];
    for word in &runic_words {
        for i in 0..scripture.len() {
            if scripture[i..].starts_with(word) {
                for j in 0..word.len() {
                    runes[i + j] = 1;
                }
            }
        }
    }
    runes.iter().sum()
}

fn parse_runic_words(notes: &str, reverse: bool) -> (Vec<String>, &str) {
    let (first, second) = notes.split_once("\n\n").unwrap();
    let mut words = Vec::new();

    for token in first[6..].split(',') {
        words.push(String::from(token));
        if reverse {
            words.push(token.chars().rev().collect());
        }
    }

    (words, second)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn quest2_part1_solution() {
        assert_eq!(quest2_part1(), 29);
    }

    #[test]
    fn quest2_part2_solution() {
        assert_eq!(quest2_part2(), 5236);
    }
}
