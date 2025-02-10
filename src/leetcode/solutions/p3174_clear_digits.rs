pub fn clear_digits(s: String) -> String {
    let mut res = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_ascii_digit() {
            res.pop();
            continue;
        }

        res.push(c);
    }

    res
}

#[cfg(test)]
mod test {

    use super::clear_digits;

    #[test]
    fn cases() {
        assert_eq!(clear_digits("ab34".into()), "");
        assert_eq!(clear_digits("ab3".into()), "a");
        assert_eq!(clear_digits("abc".into()), "abc");
    }
}
