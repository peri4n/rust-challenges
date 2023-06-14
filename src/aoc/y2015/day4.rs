use md5::compute;

const INPUT: &str = "iwrupvqb";

fn day4_fst() -> usize {
    let mut i = 0;
    loop {
        let hash = compute([INPUT, &i.to_string()].concat());
        if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4) == 0 {
            return i;
        }

        i += 1;
    }
}

fn day4_snd() -> usize {
    let mut i = 0;
    loop {
        let hash = compute([INPUT, &i.to_string()].concat());
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }

        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "To slow"]
    fn solution_fst() {
        assert_eq!(day4_fst(), 346386);
    }

    #[test]
    #[ignore = "To slow"]
    fn solution_snd() {
        assert_eq!(day4_snd(), 9958218);
    }
}
