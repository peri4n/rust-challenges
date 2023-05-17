use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    nums.into_iter().any(|n| !seen.insert(n))
}

#[cfg(test)]
mod test {
    use super::contains_duplicate;

    #[test]
    fn cases() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));

        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

}
