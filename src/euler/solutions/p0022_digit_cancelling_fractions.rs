pub fn digit_cancelling_fractions() -> u32 {
    let mut numerator_product = 1;
    let mut denominator_product = 1;

    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            let n_tens = numerator / 10;
            let n_units = numerator % 10;
            let d_tens = denominator / 10;
            let d_units = denominator % 10;

            if n_units == 0 && d_units == 0 {
                continue; // Skip trivial cases
            }

            // Check for digit cancelling
            if n_tens == d_tens && d_units != 0 && numerator * d_units == denominator * n_units {
                numerator_product *= numerator;
                denominator_product *= denominator;
            } else if n_tens == d_units
                && d_tens != 0
                && numerator * d_tens == denominator * n_units
            {
                numerator_product *= numerator;
                denominator_product *= denominator;
            } else if n_units == d_tens
                && d_units != 0
                && numerator * d_units == denominator * n_tens
            {
                numerator_product *= numerator;
                denominator_product *= denominator;
            } else if n_units == d_units
                && d_tens != 0
                && numerator * d_tens == denominator * n_tens
            {
                numerator_product *= numerator;
                denominator_product *= denominator;
            }
        }
    }

    // Reduce the fraction to its lowest terms
    let gcd = gcd(numerator_product, denominator_product);
    denominator_product / gcd
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[cfg(test)]
mod test {
    use super::digit_cancelling_fractions;

    #[test]
    fn solution() {
        assert_eq!(digit_cancelling_fractions(), 100);
    }
}
