pub fn is_subsequence(s: String, t: String) -> bool {
    let s  = s.as_bytes();
    let t = t.as_bytes();
    let mut s_index = 0_usize;
    let mut t_index = 0_usize;

    while s_index < s.len() && t_index < t.len() {
        if s[s_index] == t[t_index] {
            s_index += 1;
        }
        t_index += 1;

        if s_index == s.len() {
            return true;
        }
    }

    s_index == s.len()
}

#[cfg(test)]
mod test {
    use super::is_subsequence;

    #[test]
    fn cases() {
        assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
        assert!(!is_subsequence("acb".to_string(), "ahbgdc".to_string()));
        assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    }
}
