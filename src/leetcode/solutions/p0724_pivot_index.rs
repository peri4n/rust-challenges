pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    let mut left_sum = 0;
    for (i, n) in nums.into_iter().enumerate() {
        let right_sum = total_sum - left_sum - n;
        if left_sum == right_sum {
            return i as i32;
        }

        left_sum += n;
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn case3() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }
}
