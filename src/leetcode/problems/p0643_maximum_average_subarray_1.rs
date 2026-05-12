pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let max_sum: i32 = nums
        .windows(k as usize)
        .map(|c| c.iter().sum())
        .max()
        .unwrap();
    (max_sum as f64) / (k as f64)
}

#[cfg(test)]
mod test {

    use super::find_max_average;

    #[test]
    fn examples() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
        assert_eq!(find_max_average(vec![5], 1), 5.0);
    }
}
