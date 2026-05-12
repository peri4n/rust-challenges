pub fn convert_to_title(column_number: i32) -> String {
    let mut column_number: u32 = column_number as u32;
    let number_of_characters = 26;
    let mut result = String::new();

    while column_number > 0 {
        column_number -= 1;
        let char_code = ((column_number % number_of_characters) as u8) + b'A';
        result.push(char_code as char);

        column_number /= number_of_characters;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::convert_to_title;

    #[test]
    fn cases() {
        assert_eq!(convert_to_title(1), String::from("A"));
        assert_eq!(convert_to_title(2), String::from("B"));
        assert_eq!(convert_to_title(3), String::from("C"));
        assert_eq!(convert_to_title(26), String::from("Z"));
        assert_eq!(convert_to_title(27), String::from("AA"));
        assert_eq!(convert_to_title(28), String::from("AB"));
    }
}
