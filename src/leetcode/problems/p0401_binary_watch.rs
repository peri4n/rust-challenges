pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut times = vec![];

    for hour in 0..12 {
        for minute in 0..60 {
            let bits = i32::count_ones((hour * 64) + minute) as i32;

            if bits == turned_on {
                times.push(format!("{}:{:02}", hour, minute));
            }
        }
    }

    times
}

#[cfg(test)]
mod test {
    use super::read_binary_watch;

    #[test]
    fn cases() {
        assert_eq!(
            read_binary_watch(1),
            vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"
            ]
        );
        assert_eq!(read_binary_watch(9), Vec::<String>::new());
    }
}
