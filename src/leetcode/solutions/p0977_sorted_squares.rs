use std::collections::VecDeque;

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut result = VecDeque::with_capacity(nums.len());

    while left != right {
        if nums[left].abs() < nums[right].abs() {
            result.push_front(nums[right].pow(2));
            right -= 1
        } else {
            result.push_front(nums[left].pow(2));
            left += 1
        }
    }

    result.push_front(nums[right].pow(2));

    Vec::from(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cases() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
        assert_eq!(sorted_squares(vec![2, 3, 11]), vec![4, 9, 121]);
        assert_eq!(sorted_squares(vec![-7, -3]), vec![9, 49]);
        assert_eq!(sorted_squares(vec![-7]), vec![49]);
        assert_eq!(sorted_squares(vec![]), Vec::<i32>::new());
    }
}
