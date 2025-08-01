use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct PrimeDivisors {
    divisors_with_counts: HashMap<u32, u32>,
}

impl PrimeDivisors {
    pub fn divisors(&self) -> HashSet<u32> {
        self.divisors_with_counts.iter()
            .flat_map(|(prime, count)| vec![prime; *count as usize])
            .powerset()
            .map(|x| x.into_iter().product::<u32>())
            .collect()
    }

    pub fn merge(&mut self, other: &Self) {
        for (prime, count) in &other.divisors_with_counts {
            self.divisors_with_counts.entry(*prime)
                .and_modify(|v| *v = (*v).max(*count))
                .or_insert(*count);
        }
    }

    pub fn product(&self) -> u32 {
        self.divisors_with_counts
            .iter()
            .map(|(prime, count)| prime.pow(*count))
            .product()
    }
}

pub trait HasDivisors {
    fn divisors(&self) -> HashSet<u32>;
}

impl HasDivisors for u32 {
    fn divisors(&self) -> HashSet<u32> {
        self.prime_divisors().divisors()
    }
}

pub trait HasPrimeDivisors {
    fn prime_divisors(&self) -> PrimeDivisors;
}

impl HasPrimeDivisors for u32 {
    fn prime_divisors(&self) -> PrimeDivisors {
        let mut n = *self;
        let mut prime_divisors = HashMap::new();

        for i in 2.. {
            if n == 1 {
                break;
            }

            while n % i == 0 {
                *prime_divisors.entry(i).or_insert(0) += 1;
                n /= i;
            }
        }

        PrimeDivisors {
            divisors_with_counts: prime_divisors,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prime_divisors() {
        assert_eq!(9.prime_divisors().divisors_with_counts, HashMap::from([(3, 2)])); 
        assert_eq!(15.prime_divisors().divisors_with_counts, HashMap::from([(3, 1), (5, 1)])); 
        assert_eq!(1024.prime_divisors().divisors_with_counts, HashMap::from([(2, 10)])); 
    }

    #[test]
    fn test_all_divisors() {
        assert_eq!(9.divisors(), HashSet::from_iter(vec![1, 3, 9]));
        assert_eq!(15.divisors(), HashSet::from_iter(vec![1, 3, 5, 15]));
        assert_eq!(60.divisors(), HashSet::from_iter(vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]));
    }

    #[test]
    fn test_merge_prime_divisors() {
        let mut pd1 = 9.prime_divisors();
        let pd2 = 15.prime_divisors();

        pd1.merge(&pd2);
        assert_eq!(pd1.divisors_with_counts, HashMap::from([(3, 2), (5, 1)]));
    }
}
