use std::collections::BinaryHeap;
use std::collections::HashSet;

pub struct SmallestInfiniteSet {
    head: i32,
    added_back: BinaryHeap<i32>, // this will be used as a min-heap
    seen: HashSet<i32>,          // keeps track of duplicates
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Self {
            head: 1,
            added_back: BinaryHeap::new(),
            seen: HashSet::new(),
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if self.added_back.is_empty() {
            self.head += 1;
            self.head - 1
        } else {
            let min = -self.added_back.pop().unwrap();
            self.seen.remove(&min);
            min
        }
    }

    pub fn add_back(&mut self, num: i32) {
        if num < self.head && !self.seen.contains(&num) {
            self.seen.insert(num);
            self.added_back.push(-num);
        }
    }
}

#[cfg(test)]
mod test {
    use super::SmallestInfiniteSet;

    #[test]
    fn case1() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);
        assert_eq!(set.pop_smallest(), 3);
        set.add_back(1);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 4);
        assert_eq!(set.pop_smallest(), 5);
    }
}
