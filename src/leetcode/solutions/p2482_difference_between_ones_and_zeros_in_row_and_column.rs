pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let columns = grid[0].len();
    let rows = grid.len();

    let mut col_ones = vec![0; columns];
    let mut row_ones = vec![0; rows];

    for row in 0..rows {
        for col in 0..columns {
            col_ones[col] += grid[row][col] as usize;
            row_ones[row] += grid[row][col] as usize;
        }
    }

    let mut res = vec![vec![0_i32; columns]; rows];

    for row in 0..rows {
        let row_zeros = (columns - row_ones[row]) as i32;
        for (column, col_item) in col_ones.iter().enumerate().take(columns) {
            let col_zeros = (rows - col_item) as i32;
            res[row][column] = (row_ones[row] + col_item) as i32 - row_zeros - col_zeros;
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::ones_minus_zeros;

    #[test]
    fn testcases() {
        assert_eq!(
            ones_minus_zeros(
                vec![
                    vec![0, 1, 1], 
                    vec![1, 0, 1], 
                    vec![0, 0, 1]
                ]),
            vec![
                vec![0, 0, 4], 
                vec![0, 0, 4], 
                vec![-2, -2, 2]
            ]
        );
    }
}
