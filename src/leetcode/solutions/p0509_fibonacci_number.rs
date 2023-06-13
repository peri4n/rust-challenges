pub fn fib(n: i32) -> i32 {
    let mut prepre = 0;
    let mut pre = 1;

    for _ in 0..n {
        let pre_old = pre;
        pre = prepre + pre;
        prepre = pre_old;
    }
    prepre
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
    }
}
