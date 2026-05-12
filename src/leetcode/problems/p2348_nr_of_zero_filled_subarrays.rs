pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut total_count = 0;
    let mut curr_count = 0;

    for n in nums {
        if n == 0 {
            curr_count += 1;
            total_count += curr_count;
        } else {
            curr_count = 0
        }
    }

    total_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
    }
}
