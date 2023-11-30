pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    for (curr, next) in s.chars().zip(s[1..].chars()) {
        if to_int(next) <= to_int(curr) {
            sum += to_int(curr);
        } else {
            sum -= to_int(curr);
        }
    }
    sum + s.chars().last().map_or_else(|| 0, to_int)
}

fn to_int(literal: char) -> i32 {
    match literal {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("This should not happen"),
    }
}

#[cfg(test)]
mod test {
    use super::roman_to_int;

    #[test]
    fn case1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn case3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
