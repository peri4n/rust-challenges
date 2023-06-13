pub fn reverse_words(s: String) -> String {
    return s.split(' ')
        .map(|w| w.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ");
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn cases() {
        assert_eq!(reverse_words("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string());
        assert_eq!(reverse_words("God Ding".to_string()), "doG gniD".to_string());
    }
}
