pub fn is_ugly(mut n: i32) -> bool {
    loop {
        let old_n = n;
        for divider in [2, 3, 5] {
            if n % divider == 0 {
                n /= divider;
            }
        }

        if old_n == n {
            return n == 1;
        }
    }
}

#[cfg(test)]
mod test {

    use super::is_ugly;

    #[test]
    fn cases() {
        assert!(is_ugly(6));
        assert!(is_ugly(1));
        assert!(!is_ugly(14));
    }
}
