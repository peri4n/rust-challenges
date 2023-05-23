pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len() * 2];
    let l = nums.len();

    for (i, n) in nums.into_iter().enumerate() {
        res[i] = n;
        res[i + l] = n;
    }

    res
}

#[cfg(test)]
mod test {
    use super::get_concatenation;

    #[test]
    fn cases() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(get_concatenation(vec![1, 2, 3, 2, 1]), vec![1, 2, 3, 2, 1, 1, 2, 3, 2 ,1]);
    }
}
