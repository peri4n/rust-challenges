use crate::helper::primes::HasPrimeDivisors;

pub fn smallest_multiple() -> u32 {
    let mut prime_divisors = 2.prime_divisors();

    for i in 3..=20 {
        prime_divisors.merge(&i.prime_divisors());
    }

    prime_divisors.product()
}

#[cfg(test)]
mod test {
    use super::smallest_multiple;

    #[test]
    fn solution() {
        assert_eq!(smallest_multiple(), 232792560);
    }
}
