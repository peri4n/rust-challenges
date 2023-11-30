
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut t = x;
    let mut rev = 0;

    while t != 0 {
        rev = rev * 10 + t % 10;
        t /= 10;
    }

    rev == x
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn case1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn case2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn case3() {
        assert!(!is_palindrome(10));
    }
}
