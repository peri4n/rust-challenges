use std::collections::HashSet;

use itertools::Itertools;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let counts = arr.iter().counts();
    let unique_counts = counts.values().collect::<HashSet<_>>();

    counts.len() == unique_counts.len()
}

#[cfg(test)]
mod test {

    use super::unique_occurrences;

    #[test]
    fn examples() {
        assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(unique_occurrences(vec![1, 2]), false);
        assert_eq!(
            unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
