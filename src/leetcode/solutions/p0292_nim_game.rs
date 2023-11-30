
pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[cfg(test)]
mod test {
    use super::can_win_nim;

    #[test]
    fn cases() {
        assert_eq!(can_win_nim(4), false);
        assert_eq!(can_win_nim(2), true);
        assert_eq!(can_win_nim(1), true);
    }
}
