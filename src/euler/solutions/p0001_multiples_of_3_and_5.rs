pub fn multiples_of_3_and_5() -> i32 {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn multiples_of_3_and_5_next() -> u32 {
    let limit = 1000;
    let mut sum = 0;

    // Add multiples of 3
    let mut i = 3;
    while i < limit {
        sum += i;
        i += 3;
    }

    // Add multiples of 5 that are not also multiples of 3
    let mut j = 5;
    while j < limit {
        if j % 3 != 0 {
            sum += j;
        }
        j += 5;
    }

    sum
}

fn sum_multiples(n: u32, limit: u32) -> u32 {
    let count = (limit - 1) / n;
    n * count * (count + 1) / 2
}

pub fn multiples_of_3_and_5_next2() -> u32 {
    let limit = 1000;
    sum_multiples(3, limit) + sum_multiples(5, limit) - sum_multiples(15, limit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(multiples_of_3_and_5(), 233168);
        assert_eq!(multiples_of_3_and_5_next(), 233168);
        assert_eq!(multiples_of_3_and_5_next2(), 233168);
    }
}
