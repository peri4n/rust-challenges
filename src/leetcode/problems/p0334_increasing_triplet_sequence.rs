pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let (mut first, mut second) = (i32::MAX, i32::MAX);
    for n in nums {
        if n <= first {
            first = n;
        } else if n <= second {
            second = n;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {

    use super::increasing_triplet;

    #[test]
    fn examples() {
        assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}
