use std::collections::HashMap;

pub fn two_sum1(nums: &[i32], target: i32) -> Vec<i32> {
    let mut seen = HashMap::with_capacity(nums.len());

    for (i, n) in nums.iter().enumerate() {
        let hit = target - n;
        match seen.get(&hit) {
            Some(&j) => {
                return vec![j as i32, i as i32];
            }
            None => {
                seen.insert(n, i);
            }
        }
    }

    vec![]
}

pub fn two_sum2(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        if seen.get(&n).is_none() {
            seen.insert(n, i);
        }
    }

    for (&n, &i) in seen.iter() {
        let hit = target - n;

        if let Some(&j) = seen.get(&hit) {
            return vec![j as i32, i as i32];
        }
    }
    vec![]
}

pub fn two_sum3(nums: &[i32], target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate() {
            if n + m == target && i != j {
                return vec![j as i32, i as i32];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fmt::Debug;

    #[test]
    fn case0() {
        contain_the_same_elements(two_sum1(&vec![2, 7, 11, 15], 9), vec![0, 1]);
        contain_the_same_elements(two_sum3(&vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case1() {
        contain_the_same_elements(two_sum1(&vec![3, 2, 4], 6), vec![1, 2]);
        contain_the_same_elements(two_sum3(&vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn case2() {
        contain_the_same_elements(two_sum1(&vec![3, 3], 6), vec![0, 1]);
        contain_the_same_elements(two_sum3(&vec![3, 3], 6), vec![0, 1]);
    }

    fn contain_the_same_elements<T: Ord + Debug>(mut vec1: Vec<T>, vec2: Vec<T>) {
        vec1.sort();
        assert_eq!(vec1, vec2);
    }
}
