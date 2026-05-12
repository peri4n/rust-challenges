use crate::leetcode::problems::p0461_hamming_distance::hamming_distance;

pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for (i, m) in nums.iter().enumerate() {
        for n in &nums[i..] {
            sum += hamming_distance(*m, *n);
        }
    }
    sum
}

#[cfg(test)]
mod test {

    use super::total_hamming_distance;

    #[test]
    fn examples() {
        assert_eq!(total_hamming_distance(vec![4, 14, 2]), 6);
        assert_eq!(total_hamming_distance(vec![4, 14, 4]), 4);
    }
}
