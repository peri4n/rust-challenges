use std::fs;

const PART1_FILE: &str = "src/everybody_codes/y2024/quest2_1.txt";

pub fn quest2_part1() -> usize {
    let content =
        fs::read_to_string(PART1_FILE).expect("Should have been able to read the input file");

    let (header, scripture) = content.split_once("\n\n").unwrap();
    let runic_words = parse_runic_words(header);

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

fn parse_runic_words(line: &str) -> Vec<&str> {
    let index_of_colon = line.find(':').unwrap();
    line[index_of_colon + 1..].split(',').collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn quest2_part1_solution() {
        assert_eq!(quest2_part1(), 29);
    }
}
