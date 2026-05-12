pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut perimeter = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 1 {
                let mut land_neighbours = 0;

                // not top row
                if x != 0 {
                    land_neighbours += grid[x - 1][y];
                }

                // not left column
                if y != 0 {
                    land_neighbours += grid[x][y - 1];
                }

                // not right column
                if y != row.len() - 1 {
                    land_neighbours += grid[x][y + 1];
                }

                // not bottom row
                if x != grid.len() - 1 {
                    land_neighbours += grid[x + 1][y];
                }

                perimeter += 4 - land_neighbours;
            }
        }
    }

    perimeter
}

#[cfg(test)]
mod test {

    use super::island_perimeter;

    #[test]
    fn cases() {
        assert_eq!(
            island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
        assert_eq!(island_perimeter(vec![vec![1]]), 4);
        assert_eq!(island_perimeter(vec![vec![1, 0]]), 4);
        assert_eq!(island_perimeter(vec![]), 0);
    }
}
