pub fn prime_factors(num: u128) -> Vec<u128> {
    let mut num = num;
    let mut factors = vec![];
    if num.is_multiple_of(2) {
        factors.push(2);

        while num.is_multiple_of(2) {
            num /= 2;
        }
    }

    let mut div = 3;
    let upper_limit: u128 = (num as f64).sqrt() as u128 + 1;
    while div <= upper_limit {
        if num.is_multiple_of(div) {
            factors.push(div);

            while num.is_multiple_of(div) {
                num /= div;
            }
        }

        div += 2
    }

    factors
}

#[cfg(test)]
mod test {
    use super::prime_factors;

    #[test]
    fn solution() {
        assert_eq!(prime_factors(600851475143).last(), Some(&6857_u128));
    }
}
