pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let should = (n * (n + 1) / 2) as i32;
    should - nums.iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::missing_number;

    #[test]
    fn cases() {
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1, 3]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
