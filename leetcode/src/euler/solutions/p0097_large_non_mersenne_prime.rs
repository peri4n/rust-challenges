pub fn large_non_mersenne_prime() -> u64 {
    let mut current: u64 = 28433;
    let exponent = 7830457;
    for _ in 0..exponent {
        current *= 2;
        current %= 10_000_000_000;
    }

    current + 1
}

#[cfg(test)]
mod test {
    use super::large_non_mersenne_prime;

    #[test]
    fn solution() {
        assert_eq!(large_non_mersenne_prime(), 8739992577);
    }
}
