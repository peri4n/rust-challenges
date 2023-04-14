use std::collections::{HashMap, HashSet};

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut seen_chars = HashMap::new();
    let mut word_set = HashSet::new();
    let words: Vec<_> = s.split_ascii_whitespace().collect();

    if words.len() != pattern.len() {
        return false;
    }

    for (c, word) in pattern.chars().zip(words) {
        if let Some(old_word) = seen_chars.insert(c, word) {
            if word != old_word {
                return false;
            }
        } else if !word_set.insert(word) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert!(word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(!word_pattern(
            "aaaa".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn case4() {
        assert!(!word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string()
        ));
    }

    #[test]
    fn case5() {
        assert!(!word_pattern(
            "aaa".to_string(),
            "dog dog dog dog".to_string()
        ));
    }
}
