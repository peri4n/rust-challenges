pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    if coordinates.len() < 2 {
        return false;
    }

    let dx = coordinates[1][0] - coordinates[0][0];
    let dy = coordinates[1][1] - coordinates[0][1];

    for i in 1..coordinates.len() {
        let current_dx = coordinates[i][0] - coordinates[i - 1][0];
        let current_dy = coordinates[i][1] - coordinates[i - 1][1];
        if dx * current_dy != dy * current_dx {
            return false;
        };
    }
    true
}

#[cfg(test)]
mod test {
    use super::check_straight_line;

    #[test]
    fn cases() {
        assert!(
            check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7]
            ])
        );

        assert!(
            !check_straight_line(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![7, 7]
            ])
        );

        assert!(
            check_straight_line(vec![vec![0, 0], vec![0, 1], vec![0, -1]])
        );
    }
}
