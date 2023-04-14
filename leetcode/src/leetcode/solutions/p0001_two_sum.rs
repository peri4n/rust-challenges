use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

#[cfg(test)]
mod test {

    use super::two_sum;

    #[test]
    fn case0() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
