pub fn permute(values: Vec<i32>) -> Vec<Vec<i32>> {
    fn helper(values: &mut Vec<i32>) -> Vec<Vec<i32>> {
        if values.len() <= 1 {
            return vec![values.clone()];
        }

        let mut res = vec![];
        for _ in 0..values.len() {
            let first = values.remove(0); // pop
            let mut perms = helper(values);
            
            for p in &mut perms {
                p.push(first);
            }

            res.extend(perms);
            values.push(first); // append what has been poped
        }
        res
    }

    let mut temp = values;
    helper(&mut temp)
}

#[cfg(test)]
mod test {
    use super::permute;

    #[test]
    fn case1() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![1, 3, 2],
                vec![3, 1, 2],
                vec![2, 1, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(permute(vec![1, 0]), vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn case3() {
        assert_eq!(permute(vec![]), vec![Vec::<i32>::new()]);
    }
}
