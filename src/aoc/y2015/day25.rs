pub fn day25_fst() -> u64 {
    let mut code = 20151125u64;
    let target_row = 3010;
    let target_col = 3019;

    let mut row = 1;
    let mut column = 1;

    while row != target_row || column != target_col {
        code = (code * 252533) % 33554393;

        if row == 1 {
            row = column + 1;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }
    }

    code
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day25_fst(), 8997277);
    }
}
