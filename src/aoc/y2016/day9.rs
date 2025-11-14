pub fn day9_fst() -> usize {
    let content =
        std::fs::read_to_string("src/aoc/y2016/day9.txt").expect("Could not read input file");
    decompress_v1(&content.trim())
}

pub fn day9_snd() -> usize {
    let content =
        std::fs::read_to_string("src/aoc/y2016/day9.txt").expect("Could not read input file");
    decompress_v2(&content.trim())
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
    fn solution_fst() {
        assert_eq!(day9_fst(), 123908);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day9_snd(), 10755693147);
    }
}
