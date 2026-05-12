pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[cfg(test)]
mod test {
    use super::can_win_nim;

    #[test]
    fn cases() {
        assert!(!can_win_nim(4));
        assert!(can_win_nim(2));
        assert!(can_win_nim(1));
    }
}
