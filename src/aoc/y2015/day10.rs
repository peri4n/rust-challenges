const INPUT: &str = "1321131112";

fn run_length_encoding(str: &str) -> String {
    if str.is_empty() {
        return "".to_owned();
    }

    if str.len() == 1 {
        let mut res = String::from(str);
        res.push('1');
        return res;
    }

    let mut res = String::with_capacity(str.len());
    let characters: Vec<_> = str.bytes().collect();
    let mut occurrences = 1;

    for i in 1..characters.len() {
        if characters[i-1] == characters[i] {
            occurrences += 1;
        } else {
            res.push_str(&occurrences.to_string());
            res.push(characters[i-1] as char);
            occurrences = 1;
        }
    }

    res.push_str(&occurrences.to_string());
    res.push(characters[characters.len() - 1] as char);

    res
}

fn day10_helper(iter: usize) -> String {
    let mut str = INPUT.to_string();

    for _ in 0..iter {
        str = run_length_encoding(&str);
    }

    str
}

fn day10_fst() -> usize {
    day10_helper(40).len()
}

fn day10_snd() -> usize {
    day10_helper(50).len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(run_length_encoding("1"), "11");
        assert_eq!(run_length_encoding("11"), "21");
        assert_eq!(run_length_encoding("21"), "1211");
        assert_eq!(run_length_encoding("1211"), "111221");
        assert_eq!(run_length_encoding("111221"), "312211");
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day10_fst(), 492982);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day10_snd(), 6989950);
    }
}
