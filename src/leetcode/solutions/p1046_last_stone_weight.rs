use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);

    while heap.len() > 1 {
        let most = heap.pop().unwrap();
        let snd_most = heap.pop().unwrap();

        if most != snd_most {
            heap.push(most - snd_most);
        }
    }

    heap.pop().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::last_stone_weight;

    #[test]
    fn case1() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(last_stone_weight(vec![3, 3]), 0);
    }
}
