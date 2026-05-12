pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut res = Vec::with_capacity(nums.len());

    for i in 0..n {
        res.push(nums[i]);
        res.push(nums[i + n]);
    }

    res
}

#[cfg(test)]
mod test {
    use super::shuffle;

    #[test]
    fn cases() {
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
