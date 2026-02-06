pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let mut lll = 0;
    let mut ll = 1;
    let mut l = 1;

    let mut cur = 0;
    for _ in 0..(n - 2) {
        cur = lll + ll + l;
        lll = ll;
        ll = l;
        l = cur;
    }

    cur
}

#[cfg(test)]
mod test {

    use super::tribonacci;

    #[test]
    fn examples() {
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(25), 1389537);
    }
}
