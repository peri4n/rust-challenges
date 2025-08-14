pub struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len()];
        let mut tmp_sum = 0;

        for (i, v) in nums.into_iter().enumerate() {
            tmp_sum += v;

            sums[i] = tmp_sum;
        }

        Self { sums }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left <= 0 {
            self.sums[right as usize]
        } else {
            self.sums[right as usize] - self.sums[left as usize - 1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        let array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);

        assert_eq!(array.sum_range(0, 2), 1);
        assert_eq!(array.sum_range(2, 5), -1);
        assert_eq!(array.sum_range(0, 5), -3);
    }
}
