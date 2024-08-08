pub fn reverse_string(s: &mut [char]) {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let mut chars: Vec<char> = "hello".chars().collect();
        reverse_string(&mut chars);
        assert_eq!(chars, "olleh".chars().collect::<Vec<char>>());
    }

    #[test]
    fn case2() {
        let mut chars: Vec<char> = "Hannah".chars().collect();
        reverse_string(&mut chars);
        assert_eq!(chars, "hannaH".chars().collect::<Vec<char>>());
    }
}
