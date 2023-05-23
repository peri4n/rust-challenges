pub fn totient_permutation() -> i32 {
    let mut min = 1.0;
    let mut min_i = 0;
    for n in 1..10_000_000 {
        let p = phi(n);
        let ratio = n as f32 / p as f32;

        if n % 1000 == 0 {
            println!("{}", n);
        }

        if ratio < min && permutation(n, p) {
            println!("{} {}", min, min_i);
            min = ratio;
            min_i = n;
        }
    }

    min_i
}

pub fn permutation(mut n: i32, mut p: i32) -> bool {
    let mut tmp = 0;

    while n != 0 && p != 0{
        tmp ^= n % 10;
        tmp ^= p % 10;
        n /= 10;
        p /= 10;
    }

    tmp == 0
}

fn phi(a: i32) -> i32 {
    let mut rel_primes = 0;
    for i in 1..=a {
        if gcd(a, i) == 1 {
            rel_primes += 1
        }
    }
    rel_primes
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }

        let old_b = b;
        b = a % b;
        a = old_b;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn permutation_test() {
        assert!(permutation(13, 31));
        assert!(permutation(87109, 79180));
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(9, 6), 3);
        assert_eq!(gcd(6, 9), 3);
    }

    #[test]
    fn phi_test() {
        assert_eq!(phi(9), 6);
        assert_eq!(phi(1), 1);
    }

    #[test]
    fn solution() {
        // assert_eq!(totient_permutation(), 1);
    }
}
