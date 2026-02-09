pub fn is_perfect_square(num: i32) -> bool {
    let mut left = 1;
    let mut right = std::cmp::min(num, 46340);

    while left <= right {
        let mid = (left + right) / 2;
        let square = mid * mid;

        if square == num {
            return true;
        } else if square > num {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    false
}

#[cfg(test)]
mod test {

    use super::is_perfect_square;

    #[test]
    fn examples() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(14), false);
        assert_eq!(is_perfect_square(1), true);
        assert_eq!(is_perfect_square(808201), true);
    }
}
