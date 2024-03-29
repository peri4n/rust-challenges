pub fn hamming_weight(n: u32) -> i32 {
    let mut count = 0;
    let mut n = n;

    while n != 0 {
        n &= n - 1;
        count += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(hamming_weight(0b00000000000000000000001000000000), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
