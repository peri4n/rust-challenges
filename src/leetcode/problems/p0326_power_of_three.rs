pub fn is_power_of_three(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    while n % 3 == 0 {
        n /= 3;
    }

    n == 1
}

#[cfg(test)]
mod test {
    use super::is_power_of_three;

    #[test]
    fn cases() {
        assert!(!is_power_of_three(0));
        assert!(is_power_of_three(1));
        assert!(!is_power_of_three(2));
        assert!(is_power_of_three(3));
        assert!(is_power_of_three(9));
        assert!(is_power_of_three(27));
        assert!(!is_power_of_three(52));
    }
}
