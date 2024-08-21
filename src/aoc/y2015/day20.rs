use crate::helper::primes::FindAllDivisors;

pub fn day20_fst() -> u32 {
    let target = 29000000;
    // find the lowest number whose divisors sum is greater than or equal to the target
    let mut house = 1;
    loop {
        let sum: u32 = house.divisors().iter().sum();

        if sum * 10 >= target {
            break;
        }
        house += 1;
    }
    house
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day20_fst() {
        assert_eq!(day20_fst(), 665280);
    }
}
