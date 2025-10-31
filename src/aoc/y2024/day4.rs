use crate::helper::grid::Grid;

const INPUT_FILE: &str = "src/aoc/y2024/day4.txt";

pub fn day4_fst() -> i32 {
    let grid: Grid<char> = Grid::from_file(INPUT_FILE);
    search_for_xmas(&grid, "XMAS") as i32
}

pub fn day4_snd() -> i32 {
    let grid: Grid<char> = Grid::from_file(INPUT_FILE);
    search_for_cross_mas(&grid) as i32
}

pub fn search_for_xmas(grid: &Grid<char>, word: &str) -> u32 {
    let mut count = 0;

    for cell in grid.iter() {
        if cell.up_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.up_right_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.right_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.down_right_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.down_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.down_left_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.left_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
        if cell.up_left_iter().collect::<String>().starts_with(word) {
            count += 1;
        }
    }
    count
}

pub fn search_for_cross_mas(grid: &Grid<char>) -> u32 {
    let mut count = 0;
    for cell in grid.iter() {
        let mut left_diagonal = false;
        if cell.up_left_iter().collect::<String>().starts_with("AM")
            && cell.down_right_iter().collect::<String>().starts_with("AS")
        {
            left_diagonal = true;
        }

        if cell.up_left_iter().collect::<String>().starts_with("AS")
            && cell.down_right_iter().collect::<String>().starts_with("AM")
        {
            left_diagonal = true;
        }

        let mut right_diagonal = false;
        if cell.up_right_iter().collect::<String>().starts_with("AM")
            && cell.down_left_iter().collect::<String>().starts_with("AS")
        {
            right_diagonal = true;
        }

        if cell.up_right_iter().collect::<String>().starts_with("AS")
            && cell.down_left_iter().collect::<String>().starts_with("AM")
        {
            right_diagonal = true;
        }

        if left_diagonal && right_diagonal {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::grid::Grid;

    #[test]
    fn cases() {
        let grid = Grid::from_string(
            "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX",
        );
        assert_eq!(search_for_xmas(&grid, "XMAS"), 18);
    }

    #[test]
    fn cases_2() {
        let grid: Grid<char> = Grid::from_string(
            ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........",
        );
        assert_eq!(search_for_cross_mas(&grid), 9);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day4_fst(), 2583);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day4_snd(), 1978);
    }
}
