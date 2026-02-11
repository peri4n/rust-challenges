pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
    let mut diffs = 0;
    while x != 0 || y != 0 {
        let x_lsb = x & 1;
        let y_lsb = y & 1;

        if x_lsb != y_lsb {
            diffs += 1;
        }

        x >>= 1;
        y >>= 1;
    }

    diffs
}

#[cfg(test)]
mod test {

    use super::hamming_distance;

    #[test]
    fn examples() {
        assert_eq!(hamming_distance(1, 4), 2);
        assert_eq!(hamming_distance(3, 1), 1);
    }
}
