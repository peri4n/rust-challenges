const INPUT: usize = 29000000;

pub fn day20_fst() -> u32 {
    let mut houses = vec![0; INPUT / 10];

    for i in 1..houses.len() {
        for j in (i..houses.len()).step_by(i) {
            houses[j] += i * 10;
        }
    }

    houses.iter().position(|&x| x >= INPUT).unwrap() as u32
}

pub fn day20_snd() -> u32 {
    let mut houses = vec![0; INPUT / 10 ];

    for i in 1..houses.len() {
        for j in (i..houses.len()).step_by(i).take(50) {
            houses[j] += i * 11;
        }
    }
    houses.iter().position(|&x| x >= INPUT).unwrap() as u32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day20_fst() {
        assert_eq!(day20_fst(), 665280);
    }

    #[test]
    fn test_day20_snd() {
        assert_eq!(day20_snd(), 705600);
    }
}
