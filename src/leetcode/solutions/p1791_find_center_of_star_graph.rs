pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut counts = vec![0; edges.len() + 1];

    for edge in edges {
        let from = (edge[0] - 1) as usize;
        let to = (edge[1] - 1) as usize;

        counts[from] += 1;
        if counts[from] > 1 {
            return (from + 1) as i32;
        }

        counts[to] += 1;
        if counts[to] > 1 {
            return (to + 1) as i32;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::find_center;

    #[test]
    fn cases() {
        assert_eq!(find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
        assert_eq!(find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 5]]), 1);
    }
}
