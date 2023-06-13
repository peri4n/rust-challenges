pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if is_multiple(&str1, &str2) {
        let g = gcd(str1.len(), str2.len());
        return str1[0..g].to_owned();
    }

    "".to_string()
}

fn is_multiple(str1: &str, str2: &str) -> bool {
    let chars1 = str1.chars();
    let chars2 = str2.chars();

    chars1.clone().chain(chars2.clone()).eq(chars2.chain(chars1))
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    loop {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }

        let remainder = a % b;
        a = b;
        b = remainder;
    }
}

#[cfg(test)]
mod test {
    use super::gcd_of_strings;

    #[test]
    fn cases() {
        assert_eq!(gcd_of_strings("ABCABC".to_string(), "ABC".to_string()), "ABC".to_string());
        assert_eq!(gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()), "AB".to_string());
        assert_eq!(gcd_of_strings("LEET".to_string(), "CODE".to_string()), "".to_string());
    }
}
