use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut seen = HashSet::new();

    let mut res = square_sum(n);
    while res != 1 {
        if seen.insert(res) {
            res = square_sum(res);
        } else {
            return false;
        }
    }
    true
}

fn square_sum(n: i32) -> i32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let x = n % 10;
        sum += x * x;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::is_happy;

    #[test]
    fn case1() {
        assert!(is_happy(1));
    }

    #[test]
    fn case2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn case3() {
        assert!(is_happy(19));
    }
}
