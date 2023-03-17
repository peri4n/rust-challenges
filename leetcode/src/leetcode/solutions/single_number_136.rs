pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .reduce(|x, y| x ^ y)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::single_number;

    #[test]
    fn case0() {
        assert_eq!(single_number(vec![]), 0);
    }

    #[test]
    fn case1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(single_number(vec![1]), 1);
    }
}
