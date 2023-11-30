pub fn is_right_triangle(a: usize, b: usize, c: usize) -> bool {
    a * a + b * b == c * c
}

pub fn biggest_perimeter(upper_limit: usize) -> usize {
    let half_limit = upper_limit / 2;
    let mut perimeters = vec![0; upper_limit];

    for a in 1..half_limit {
        for b in a..half_limit - 1 {
            for c in b..(upper_limit - a - b) {
                if is_right_triangle(a, b, c) {
                    perimeters[a + b + c] += 1;
                }
            }
        }
    }
    perimeters
        .into_iter()
        .enumerate()
        .max_by_key(|f| f.1)
        .unwrap()
        .0
}

#[cfg(test)]
mod test {
    use super::biggest_perimeter;

    #[test]
    fn solution() {
        assert_eq!(biggest_perimeter(1000), 840);
    }
}
