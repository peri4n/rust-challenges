
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = HashSet::from_iter(nums1);
    let set2: HashSet<i32> = HashSet::from_iter(nums2);

    set1.intersection(&set2).copied().collect()
}

#[cfg(test)]
mod test {
    use super::intersection;

    #[test]
    fn cases() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2]), vec![2]);
        let mut inter = intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        inter.sort();
        assert_eq!(inter, vec![4, 9]);
    }
}
