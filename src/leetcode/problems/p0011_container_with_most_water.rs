pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut max = 0;

    while i < j {
        let curr_volumn = height[i].min(height[j]) * (j - i) as i32;
        if curr_volumn > max {
            max = curr_volumn;
        }
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    max
}

#[cfg(test)]
mod test {
    use super::max_area;

    #[test]
    fn case1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn case2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
