pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut count_negatives = 0;

    for n in nums {
        if n == 0 {
            return 0;
        } else if n < 0 {
            count_negatives += 1;
        }
    }

    if count_negatives % 2 == 0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::array_sign;

    #[test]
    fn cases() {
        assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
