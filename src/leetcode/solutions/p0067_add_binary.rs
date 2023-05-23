pub fn add_binary(a: String, b: String) -> String {
        let chars_a: Vec<_> = a.chars().collect();
        let chars_b: Vec<_> = b.chars().collect();

        let mut i = (chars_a.len() - 1) as i32;
        let mut j = (chars_b.len() - 1) as i32;
        let mut res = String::from("");
        let mut carry = 0;
        
        while i >= 0 || j >= 0 {
            let char_a = if i >= 0 { chars_a[i as usize] } else { '0' };
            let char_b = if j >= 0 { chars_b[j as usize] } else { '0' };

            if char_a == '1' && char_b == '1' && carry == 0 {
                res.push('0');
                carry = 1;
                i -= 1;
                j -= 1;
                continue;
            }

            if char_a == '1' && char_b == '1' && carry == 1 {
                res.push('1');
                carry = 1;
                i -= 1;
                j -= 1;
                continue;
            }

            if ((char_a == '1') ^ (char_b == '1')) && carry == 1 {
                res.push('0');
                carry = 1;
                i -= 1;
                j -= 1;
                continue;
            }

            if ((char_a == '1') ^ (char_b == '1')) && carry == 0 {
                res.push('1');
                carry = 0;
                i -= 1;
                j -= 1;
                continue;
            }

            if ((char_a == '0') && (char_b == '0')) && carry == 1 {
                res.push('1');
                carry = 0;
                i -= 1;
                j -= 1;
                continue;
            }

            if ((char_a == '0') && (char_b == '0')) && carry == 0 {
                res.push('0');
                carry = 0;
                i -= 1;
                j -= 1;
                continue;
            }
        }
        if carry == 1 {
            res.push('1');
        }
        res.chars().rev().collect()
    }

#[cfg(test)]
mod test {
    use super::add_binary;

    #[test]
    fn case1() {
        assert_eq!(add_binary("1".to_string(), "11".to_string()), "100".to_string());
    }
    #[test]
    fn case2() {
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
    }
}
