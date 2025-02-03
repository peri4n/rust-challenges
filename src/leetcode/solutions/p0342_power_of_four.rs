pub fn is_power_of_four(n: i32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn cases() {
        assert!(!is_power_of_four(0));
        assert!(is_power_of_four(1));
        assert!(!is_power_of_four(2));
        assert!(!is_power_of_four(3));
        assert!(is_power_of_four(4));
        assert!(!is_power_of_four(5));
        assert!(is_power_of_four(16));
    }
}
