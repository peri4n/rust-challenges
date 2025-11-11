const INPUT_FILE: &str = "src/aoc/y2017/day9.txt";

fn input() -> String {
    std::fs::read_to_string(INPUT_FILE).expect("Could not read input file")
}

pub fn day9_fst() -> usize {
    let input = input();
    let mut score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match c {
            '!' if in_garbage => skip_next = true,
            '<' if !in_garbage => in_garbage = true,
            '>' if in_garbage => in_garbage = false,
            '{' if !in_garbage => depth += 1,
            '}' if !in_garbage => {
                score += depth;
                depth -= 1;
            }
            _ => {}
        }
    }

    score
}

pub fn day9_snd() -> usize {
    let input = input();
    let mut garbage_count = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match c {
            '!' if in_garbage => skip_next = true,
            '<' if !in_garbage => in_garbage = true,
            '>' if in_garbage => in_garbage = false,
            _ if in_garbage => garbage_count += 1,
            _ => {}
        }
    }

    garbage_count
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day9_fst(), 14204);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day9_snd(), 6622);
    }
}
