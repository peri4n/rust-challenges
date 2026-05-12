pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .flat_map(|n| n.to_string().chars().collect::<Vec<_>>())
        .map(|c: char| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        assert_eq!(separate_digits(vec![1, 23, 456]), vec![1, 2, 3, 4, 5, 6]);
    }
}
