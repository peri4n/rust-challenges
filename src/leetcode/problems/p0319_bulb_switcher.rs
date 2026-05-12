pub fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}

#[cfg(test)]
mod test {
    use super::bulb_switch;

    #[test]
    fn cases() {
        assert_eq!(bulb_switch(0), 0);
        assert_eq!(bulb_switch(1), 1);
        assert_eq!(bulb_switch(3), 1);
    }
}
