pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .chars()
        .map(|c| c as i32 - 'A' as i32 + 1)
        .fold(0, |prev, next| prev * 26 + next)
}

#[cfg(test)]
mod test {
    use super::title_to_number;

    #[test]
    fn cases() {
        assert_eq!(title_to_number("A".into()), 1);
        assert_eq!(title_to_number("B".into()), 2);
        assert_eq!(title_to_number("AB".into()), 28);
        assert_eq!(title_to_number("ZY".into()), 701);
    }
}
