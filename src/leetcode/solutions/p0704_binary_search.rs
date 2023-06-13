pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = (nums.len() as i32) - 1;

    while right >= left {
        let pivot = (right + left) / 2;
        let mid = nums[pivot as usize];
        if target == mid {
            return pivot;
        }

        if mid <= target {
            left = pivot + 1;
        } else if mid > target {
            right = pivot - 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(search(vec![2, 5], 5), 1);
        assert_eq!(search(vec![5], 5), 0);
        assert_eq!(search(vec![], 5), -1);
    }
}
