pub fn find_the_difference(s: String, t: String) -> char {
    let mut res = 0;
    for b in s.bytes() {
        res ^= b;
    }

    for b in t.bytes() {
        res ^= b;
    }

    res as char
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(find_the_difference("abcd".into(), "abcde".into()), 'e');
        assert_eq!(find_the_difference("".into(), "y".into()), 'y');
    }
}
