pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut chars_1 = word1.chars();
    let mut chars_2 = word2.chars();
    let mut result = String::with_capacity(word1.len() + word2.len());
    for _ in 0..word1.len().min(word2.len()) {
        result.push(chars_1.next().unwrap());
        result.push(chars_2.next().unwrap());
    }
    result.extend(chars_1);
    result.extend(chars_2);
    result
}

#[cfg(test)]
mod test {
    use super::merge_alternately;

    #[test]
    fn cases() {
        assert_eq!(merge_alternately(String::from("abc"), String::from("pqr")), String::from("apbqcr"));
        assert_eq!(merge_alternately(String::from("ab"), String::from("pqrs")), String::from("apbqrs"));
        assert_eq!(merge_alternately(String::from("abcd"), String::from("pq")), String::from("apbqcd"));
    }
}
