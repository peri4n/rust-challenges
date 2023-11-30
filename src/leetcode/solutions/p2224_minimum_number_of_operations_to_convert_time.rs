pub fn convert_time(current: String, correct: String) -> i32 {
    let time1_parts: Vec<_> = current.split(':').collect();
    let time2_parts: Vec<_> = correct.split(':').collect();

    let time1_hour = time1_parts[0].parse::<i32>().unwrap();
    let time1_minutes = time1_parts[1].parse::<i32>().unwrap();

    let time2_hour = time2_parts[0].parse::<i32>().unwrap();
    let time2_minutes = time2_parts[1].parse::<i32>().unwrap();

    let hour_diff = time2_hour - time1_hour;
    let minutes_diff = time2_minutes - time1_minutes;

    let mut operations = 0;
    let mut minutes = hour_diff * 60 + minutes_diff;
    while minutes != 0 {
        if minutes >= 60 {
            minutes -= 60;
            operations += 1;
            continue;
        }

        if minutes >= 15 {
            minutes -= 15;
            operations += 1;
            continue;
        }

        if minutes >= 5 {
            minutes -= 5;
            operations += 1;
            continue;
        }

        if minutes >= 1 {
            minutes -= 1;
            operations += 1;
            continue;
        }
    }
    operations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(convert_time("02:30".to_string(), "04:35".to_string()), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(convert_time("11:00".to_string(), "11:01".to_string()), 1);
    }
}
