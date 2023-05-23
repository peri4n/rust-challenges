use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let set1: HashSet<i32> = HashSet::from_iter(nums1);
    let set2: HashSet<i32> = HashSet::from_iter(nums2);

    let vec1: Vec<i32> = set1.difference(&set2).copied().collect();
    let vec2: Vec<i32> = set2.difference(&set1).copied().collect();

    vec![vec1, vec2]
}

#[cfg(test)]
mod test {
    use super::find_difference;

    #[test]
    fn cases() {
        let diff = find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        assert!(diff[0].contains(&1));
        assert!(diff[0].contains(&3));
        assert_eq!(diff[0].len(), 2);
        assert!(diff[1].contains(&4));
        assert!(diff[1].contains(&6));
        assert_eq!(diff[1].len(), 2);
    }
}
