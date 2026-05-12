pub fn permute(values: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let len = values.len();

    if len > 0 {
        permute_helper(&mut res, values, 0, len - 1);
    }

    res
}

fn permute_helper(res: &mut Vec<Vec<i32>>, mut values: Vec<i32>, left: usize, right: usize) {
    if left == right {
        res.push(values.clone());
        return;
    }

    for i in left..=right {
        values.swap(left, i);
        permute_helper(res, values.clone(), left + 1, right);
        values.swap(left, i);
    }
}

#[cfg(test)]
mod test {
    use super::permute;
    use googletest::prelude::*;

    #[test]
    fn case1() {
        assert_eq!(permute(vec![1, 2, 3]).len(), 6);
        assert_that!(
            permute(vec![1, 2, 3]),
            unordered_elements_are![
                &vec![3, 2, 1],
                &vec![2, 3, 1],
                &vec![1, 3, 2],
                &vec![3, 1, 2],
                &vec![2, 1, 3],
                &vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn case2() {
        assert_that!(
            permute(vec![1, 0]),
            unordered_elements_are![&vec![1, 0], &vec![0, 1]]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(permute(vec![]), Vec::<Vec<i32>>::new());
    }
}
