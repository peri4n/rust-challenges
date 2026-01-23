pub fn compress(chars: &mut Vec<char>) -> i32 {
    let n = chars.len();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < n {
        while i < n && chars[i] == chars[j] {
            i += 1;
        }

        chars[k] = chars[i - 1];
        k += 1;

        if i > j + 1 {
            for c in (i - j).to_string().chars() {
                chars[k] = c;
                k += 1;
            }
        }
        j = i;
    }
    k as i32
}

#[cfg(test)]
mod test {

    use super::compress;

    #[test]
    fn examples() {
        assert_eq!(compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']), 6); // a2b2c3
        assert_eq!(compress(&mut vec!['a']), 1); // a
        assert_eq!(
            compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ]),
            4
        ); // ab12
    }
}
