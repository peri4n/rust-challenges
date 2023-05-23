pub fn prime_factors(num: u128) -> Vec<u128> {
    let mut num = num;
    let mut factors = vec![];
    if num % 2 == 0 {
        factors.push(2);

        while num % 2 == 0 {
            num /= 2;
        }
    }

    let mut div = 3;
    let upper_limit: u128 = (num as f64).sqrt() as u128 + 1;
    while div <= upper_limit {
        if num % div == 0 {
            factors.push(div);

            while num % div == 0 {
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
