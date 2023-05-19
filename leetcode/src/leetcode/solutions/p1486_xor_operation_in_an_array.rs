pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut res = 0;
    for i in 0..n {
        res ^= start + 2 * i;
    }

    res
}

#[cfg(test)]
mod test {
    use super::xor_operation;

    #[test]
    fn cases() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
    }
}
