pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut buffer = String::new();
    generate_parenthesis_helper(&mut res, &mut buffer, 0, 0, n as u32);
    res
}

fn generate_parenthesis_helper(
    res: &mut Vec<String>,
    buffer: &mut String,
    open: u32,
    close: u32,
    n: u32,
) {
    if (open == n) && (close == n) {
        res.push(buffer.clone());
        return;
    }

    if open < n {
        buffer.push('(');
        generate_parenthesis_helper(res, buffer, open + 1, close, n);
        buffer.pop();
    }

    if close < open {
        buffer.push(')');
        generate_parenthesis_helper(res, buffer, open, close + 1, n);
        buffer.pop();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        assert_eq!(generate_parenthesis(3).len(), 5);
    }
}
