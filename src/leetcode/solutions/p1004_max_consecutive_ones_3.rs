pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut zero_count = 0;
    let mut left = 0;
    let mut max_length = 0;

    for right in 0..nums.len() {
        if nums[right] == 0 {
            zero_count += 1;
        }

        while zero_count > k {
            if nums[left] == 0 {
                zero_count -= 1;
            }

            left += 1;
        }

        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod test {

    use super::longest_ones;

    #[test]
    fn examples() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0], 3), 8);
        assert_eq!(
            longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
