pub fn add_digits(num: i32) -> i32 {
    let mut sum = digit_sum(num);
    while sum >= 10 {
        sum = digit_sum(sum);
    }
    sum
}

fn digit_sum(num: i32) -> i32 {
    let mut num = num;
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(add_digits(38), 2);
        assert_eq!(add_digits(0), 0);
        assert_eq!(add_digits(10), 1);
    }

    #[test]
    fn digit_sum_test() {
        assert_eq!(digit_sum(38), 11);
        assert_eq!(digit_sum(11), 2);

        assert_eq!(digit_sum(10), 1);
    }
}
