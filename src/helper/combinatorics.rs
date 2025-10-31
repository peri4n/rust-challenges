pub fn permute<T: Clone + Copy>(values: Vec<T>) -> Vec<Vec<T>> {
    fn helper<T: Clone + Copy>(values: &mut Vec<T>) -> Vec<Vec<T>> {
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

    let mut temp = values.clone();
    helper(&mut temp)
}
