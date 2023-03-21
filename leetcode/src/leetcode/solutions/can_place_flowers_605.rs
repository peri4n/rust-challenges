pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut i = 0;
    let mut slots = 0;
    let nr_slots = flowerbed.len();

    while i < nr_slots {
        let pre = if i == 0 { 0 } else { flowerbed[i - 1] };
        let curr = flowerbed[i];
        let next = if i == nr_slots - 1 { 0 } else { flowerbed[i + 1] };

        if pre == 0 && curr == 0 && next == 0 {
            slots += 1;
            i += 1;
        }

        i += 1
    }

    slots >= n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    }

    #[test]
    fn case2() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }

    #[test]
    fn case3() {
        assert_eq!(can_place_flowers(vec![0, 0, 1, 0, 1], 1), true);
    }

    #[test]
    fn case4() {
        assert_eq!(can_place_flowers(vec![1, 0, 1, 0, 0], 1), true);
    }

    #[test]
    fn case5() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 0], 2), true);
    }
}
