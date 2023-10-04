pub fn smallest_even_multiple(n: i32) -> i32 {
    if n & 1 == 1 {
        2 * n
    } else {
        n
    }
}

#[cfg(test)]
mod test {
    use super::smallest_even_multiple;

    #[test]
    fn cases() {
        assert_eq!(smallest_even_multiple(5), 10);
        assert_eq!(smallest_even_multiple(6), 6);
    }
}
