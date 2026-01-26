use std::hash::{DefaultHasher, Hash, Hasher};

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let col_hashs = (0..grid.len()).map(|col_idx| {
        let mut hasher = DefaultHasher::new();
        for row_idx in 0..grid.len() {
            grid[row_idx][col_idx].hash(&mut hasher);
        }
        hasher.finish()
    });

    let row_hashs = grid.iter().map(|row| {
        let mut hasher = DefaultHasher::new();
        for &val in row {
            val.hash(&mut hasher);
        }
        hasher.finish()
    });

    let mut count = 0;
    for row_hash in row_hashs {
        for col_hash in col_hashs.clone() {
            if row_hash == col_hash {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {

    use super::equal_pairs;

    #[test]
    fn examples() {
        assert_eq!(
            equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
        assert_eq!(
            equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }
}
