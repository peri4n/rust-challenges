pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![nums[0].to_string()];
    }

    let mut ranges = vec![];

    let mut left = nums[0];
    let mut last = left;
    for i in nums.into_iter().skip(1) {
        if i - last != 1 {
            if left == last {
                ranges.push(format!("{}", left));
            } else {
                ranges.push(format!("{}->{}", left, last));
            }
            left = i;
        }
        last = i;
    }

    if left == last {
        ranges.push(format!("{}", left));
    } else {
        ranges.push(format!("{}->{}", left, last));
    }

    ranges
}

#[cfg(test)]
mod test {

    use super::summary_ranges;

    #[test]
    fn cases() {
        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }
}
