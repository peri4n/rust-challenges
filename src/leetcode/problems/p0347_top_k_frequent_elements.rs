use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();
    let nums_len = nums.len();
    for n in nums {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut sorted_counts = vec![vec![]; nums_len];

    for (k, v) in counts {
        sorted_counts[v - 1].push(k);
    }

    let mut res = vec![];
    for s in sorted_counts.into_iter().rev() {
        for c in s {
            res.push(c);

            if res.len() as i32 == k {
                return res;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::top_k_frequent;

    #[test]
    fn cases() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
