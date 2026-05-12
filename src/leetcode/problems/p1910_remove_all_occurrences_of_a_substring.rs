pub fn remove_occurrences(mut s: String, part: String) -> String {
    while let Some(offset) = s.find(&part) {
        s.replace_range(offset..(offset + part.len()), "");
    }

    s
}

#[cfg(test)]
mod test {

    use super::remove_occurrences;

    #[test]
    fn cases() {
        assert_eq!(
            remove_occurrences("daabcbaabcbc".into(), "abc".into()),
            "dab"
        );
        assert_eq!(remove_occurrences("axxxxyyyyb".into(), "xy".into()), "ab");
    }
}
