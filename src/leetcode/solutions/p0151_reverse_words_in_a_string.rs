pub fn reverse_words(s: String) -> String {
    s.trim()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod test {

    use super::reverse_words;

    #[test]
    fn examples() {
        assert_eq!(
            reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
        assert_eq!(
            reverse_words("  Bob    Loves  Alice   ".to_string()),
            "Alice Loves Bob".to_string()
        );
        assert_eq!(
            reverse_words("Alice does not even like bob".to_string()),
            "bob like even not does Alice".to_string()
        );
    }
}
