pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
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
