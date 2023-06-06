pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = (nums.len() as i32) - 1;
        let mut ans = -1;

        while right >= left {
            let pivot = (right + left) / 2;
            let mid = nums[pivot as usize];

            if mid == target {
                return pivot;
            } else if mid < target {
                left = pivot + 1;
                ans = pivot + 1;
            } else {
                right = pivot - 1;
                ans = pivot;
            }
        }

        return ans;
    }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cases() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
