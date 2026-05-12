use num::Integer;

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return String::new(); // Early termination: Condition for no common divisor
    }

    let len1 = str1.len();
    let len2 = str2.len();
    let g = len1.gcd(&len2);

    str1[..g].to_string()
}

#[cfg(test)]
mod test {
    use super::gcd_of_strings;

    #[test]
    fn cases() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }
}
