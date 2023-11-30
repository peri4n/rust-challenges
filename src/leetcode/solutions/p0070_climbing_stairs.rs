pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut prepre = 1;
    let mut pre = 2;

    for _ in 0..(n - 2) {
        let pre_old = pre;
        pre += prepre;
        prepre = pre_old;
    }

    pre
}

#[cfg(test)]
mod test {
    use super::climb_stairs;

    #[test]
    fn cases() {
        assert_eq!(climb_stairs(0), 0);
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
        assert_eq!(climb_stairs(6), 13);
    }
}
