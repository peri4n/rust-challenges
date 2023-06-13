pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' if Some(c) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("{[]}".to_string()), true);

        assert_eq!(is_valid("([".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
    }
}
