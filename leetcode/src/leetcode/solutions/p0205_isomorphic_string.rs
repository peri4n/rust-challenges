use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let t_chars: Vec<char> = t.chars().collect();

    let mut seen = HashMap::new();
    for (i, c) in s.char_indices() {
        if let Some(&x) = seen.get(&c) {
            if x != t_chars[i] {
                return false;
            }
        } else if seen.values().any(|&x| x == t_chars[i]) {
            return false;
        } else {
            seen.insert(c, t_chars[i]);
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            is_isomorphic(String::from("egg"), String::from("add")),
            true
        );
    }

    #[test]
    fn cases2() {
        assert_eq!(
            is_isomorphic(String::from("foo"), String::from("bar")),
            false
        );
    }

    #[test]
    fn cases3() {
        assert_eq!(
            is_isomorphic(String::from("paper"), String::from("title")),
            true
        );
    }

    #[test]
    fn cases4() {
        assert_eq!(
            is_isomorphic(String::from("badc"), String::from("baba")),
            false
        );
    }
}
