pub fn reverse_vowels(s: String) -> String {
    let is_vowel = |b: u8| {
        matches!(
            b,
            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
        )
    };

    let mut vb = s.into_bytes();
    let (mut l, mut r) = (0, vb.len() - 1);
    while l < r {
        while l < r && !is_vowel(vb[l]) {
            l += 1;
        }
        while l < r && !is_vowel(vb[r]) {
            r -= 1;
        }
        if l < r {
            vb.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
    String::from_utf8(vb).unwrap()
}

#[cfg(test)]
mod test {
    use crate::leetcode::solutions::p0345_reverse_vowels_of_a_string::reverse_vowels;

    #[test]
    fn examples() {
        assert_eq!(reverse_vowels("hanno".to_string()), "honna".to_string());
        assert_eq!(
            reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        );
        assert_eq!(reverse_vowels("a.".to_string()), "a.".to_string());
    }
}
