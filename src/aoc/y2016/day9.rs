use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day9.txt");

pub struct Day9 {
    compressed: String,
}

impl Solution for Day9 {
    type Fst = usize;
    type Snd = usize;

    fn parse(input: &str) -> Self {
        Day9 {
            compressed: input.trim().to_string(),
        }
    }

    fn fst(&self) -> usize {
        decompress_v1(&self.compressed)
    }

    fn snd(&self) -> usize {
        decompress_v2(&self.compressed)
    }
}

fn decompress_v1(input: &str) -> usize {
    let mut total_length = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let mut j = i + 1;
            while chars[j] != ')' {
                j += 1;
            }
            let marker: String = chars[i + 1..j].iter().collect();
            let parts: Vec<&str> = marker.split('x').collect();
            let length: usize = parts[0].parse().unwrap();
            let count: usize = parts[1].parse().unwrap();

            let start = j + 1;
            let end = start + length;
            let segment: String = chars[start..end].iter().collect();

            total_length += segment.len() * count;

            i = end;
        } else {
            total_length += 1;
            i += 1;
        }
    }

    total_length
}

fn decompress_v2(input: &str) -> usize {
    let mut total_length = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let mut j = i + 1;
            while chars[j] != ')' {
                j += 1;
            }
            let marker: String = chars[i + 1..j].iter().collect();
            let parts: Vec<&str> = marker.split('x').collect();
            let length: usize = parts[0].parse().unwrap();
            let count: usize = parts[1].parse().unwrap();

            let start = j + 1;
            let end = start + length;
            let segment: String = chars[start..end].iter().collect();

            let segment_length = decompress_v2(&segment);
            total_length += segment_length * count;

            i = end;
        } else {
            total_length += 1;
            i += 1;
        }
    }

    total_length
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day9::parse(INPUT);
        assert_eq!(day.fst(), 123908);
        assert_eq!(day.snd(), 10755693147);
    }
}
