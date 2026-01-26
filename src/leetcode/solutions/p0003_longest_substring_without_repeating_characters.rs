use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    let mut res = 0;

    let s_chars: Vec<char> = s.chars().collect();

    while r < s_chars.len() {
        while char_set.contains(&s_chars[r]) {
            char_set.remove(&s_chars[l]);
            l += 1;
        }
        char_set.insert(s_chars[r]);
        res = res.max(r - l + 1);
        r += 1;
    }

    res as i32
}
