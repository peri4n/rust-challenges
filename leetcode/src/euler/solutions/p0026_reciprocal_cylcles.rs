use std::collections::HashMap;

pub fn cycle_length(b: i32) -> i32 {
    let mut a: i32 = 1;
    let mut t: i32 = 0;
    let mut seen = HashMap::new();
    loop {
        if a == 0 {
            return 0;
        }
        match seen.get(&a) {
            Some(&x) => return t - x,
            None => {
                seen.insert(a, t);
                a = a * 10 % b;
            }
        }

        t += 1;
    }
}

#[cfg(test)]
mod test {
    use super::cycle_length;

    #[test]
    fn correct_cycle_length() {
        assert_eq!(cycle_length(1), 0); // .0
        assert_eq!(cycle_length(2), 0); // .5
        assert_eq!(cycle_length(3), 1); // .(3)*
        assert_eq!(cycle_length(4), 0); // .25
        assert_eq!(cycle_length(5), 0); // .2
        assert_eq!(cycle_length(6), 1); // .1(6)*
        assert_eq!(cycle_length(7), 6); // .(142857)*
        assert_eq!(cycle_length(8), 0); // .125
        assert_eq!(cycle_length(9), 1); // .(1)*
    }

    #[test]
    fn solution() {
        assert_eq!((1..1000).max_by_key(|&d| cycle_length(d)), Some(983));
    }
}
