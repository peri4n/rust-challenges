pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.into_iter().filter(|&hour| hour >= target).count() as _
}

#[cfg(test)]
mod test {
    use super::number_of_employees_who_met_target;

    #[test]
    fn cases() {
        assert_eq!(
            number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2),
            3
        );
        assert_eq!(
            number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6),
            0
        );
    }
}
