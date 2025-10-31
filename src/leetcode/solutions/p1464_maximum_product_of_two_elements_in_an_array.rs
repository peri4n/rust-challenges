/// TODO This can be improved by only storing the two largest values in the heap
use std::collections::BinaryHeap;

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(nums);

    (heap.pop().unwrap() - 1) * (heap.pop().unwrap() - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(max_product(vec![3, 7]), 12);
    }
}
