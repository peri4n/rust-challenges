pub struct Fib {
    previous: i32,
    current: i32,
}

impl Fib {
    pub fn new() -> Self {
        Self { previous: 0, current: 1 }
    }
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.current + self.previous;
        self.previous = self.current;
        self.current = sum;

        Some(self.previous)
    }
}

pub fn even_fibonacci() -> i32 {
    Fib::new()
        .take_while(|&f| f < 4_000_000)
        .filter(|&f| f % 2 == 0)
        .sum()
}

#[cfg(test)]
mod test {
    use super::even_fibonacci;

    #[test]
    fn solution() {
        assert_eq!(even_fibonacci(), 4613732);
    }
}
