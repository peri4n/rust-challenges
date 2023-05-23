pub fn multiples_of_3_and_5() -> i32 {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod test {
    use super::multiples_of_3_and_5;

    #[test]
    fn solution() {
        assert_eq!(multiples_of_3_and_5(), 233168);
    }
}
