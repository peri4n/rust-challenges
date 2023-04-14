use std::collections::HashMap;

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashMap::with_capacity(nums.len());

        for (i, val) in nums.iter().enumerate() {
            if let Some(old_i) = seen.insert(val, i) {
                if i - old_i <= k as usize {
                    return true;
                }
            }
        }
        return false;
    }

#[cfg(test)]
mod test {
    use super::contains_nearby_duplicate;

    #[test]
    fn case1() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn case2() {
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 2));
    }

    #[test]
    fn case3() {
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
