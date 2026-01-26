use std::collections::HashSet;

use itertools::Itertools;

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let char_set1 = word1.chars().collect::<HashSet<_>>();
    let char_set2 = word2.chars().collect::<HashSet<_>>();

    if char_set1 != char_set2 {
        return false;
    }

    let counts1 = word1.chars().counts();
    let counts1_sorted = counts1.values().sorted().collect::<Vec<_>>();
    let counts2 = word2.chars().counts();
    let counts2_sorted = counts2.values().sorted().collect::<Vec<_>>();

    counts1_sorted == counts2_sorted
}

#[cfg(test)]
mod test {

    use super::close_strings;

    #[test]
    fn exmaples() {
        assert_eq!(close_strings("abc".to_string(), "bca".to_string()), true);
        assert_eq!(close_strings("a".to_string(), "aa".to_string()), false);
        assert_eq!(close_strings("cabbba".to_string(), "abbccc".to_string()), true);
        assert_eq!(close_strings("uau".to_string(), "ssx".to_string()), false);
    }
}
