use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut res = KthLargest {
            k: k as usize,
            heap: BinaryHeap::with_capacity(k as usize + 1),
        };

        for n in nums {
            res.add(n);
        }

        res
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);

        if self.heap.len() > self.k {
            self.heap.pop();
        }

        -*self.heap.peek().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::KthLargest;

    #[test]
    fn case() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
