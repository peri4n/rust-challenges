use crate::aoc::Solution;

const PWD_LENGTH: usize = 8;
type Cipher = [char; PWD_LENGTH];

pub const INPUT: &str = "cqjxjnds";

struct Password {
    cipher: Cipher,
}

impl Password {
    fn new(cipher: Cipher) -> Self {
        Self { cipher }
    }
}

impl Iterator for Password {
    type Item = Cipher;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 7;
        loop {
            if self.cipher[i] != 'z' {
                self.cipher[i] = (self.cipher[i] as u8 + 1) as char;
                return Some(self.cipher);
            }
            self.cipher[i] = 'a';
            i -= 1;
        }
    }
}

fn contains_iol(pwd: &Cipher) -> bool {
    for c in *pwd {
        if c == 'i' || c == 'l' {
            return true;
        }
    }

    false
}

fn contains_two_pair(pwd: &Cipher) -> bool {
    let mut found_first = false;

    let mut i = 0;
    while i < PWD_LENGTH - 1 {
        if pwd[i] == pwd[i + 1] {
            if !found_first {
                found_first = true;
                i += 1;
            } else {
                return true;
            }
        }

        i += 1;
    }

    false
}

fn contains_triplet(pwd: &Cipher) -> bool {
    for i in 0..PWD_LENGTH - 2 {
        let curr = pwd[i] as i8;
        let next = pwd[i + 1] as i8;
        let next_next = pwd[i + 2] as i8;

        if next_next - next == 1 && next - curr == 1 {
            return true;
        }
    }
    false
}

fn valid(pwd: &Cipher) -> bool {
    contains_triplet(pwd) && !contains_iol(pwd) && contains_two_pair(pwd)
}

pub struct Day11 {
    cipher: Cipher,
}

impl Solution for Day11 {
    type Fst = Cipher;
    type Snd = Cipher;

    fn parse(input: &str) -> Self {
        let cipher = input
            .trim()
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Password must have 8 characters");
        Day11 { cipher }
    }

    fn fst(&self) -> Cipher {
        Password::new(self.cipher).find(valid).unwrap()
    }

    fn snd(&self) -> Cipher {
        Password::new(self.cipher).filter(valid).nth(1).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_iterator() {
        let mut pwd = Password::new(Day11::parse(INPUT).cipher);
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 't']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'u']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'v']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'w']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'x']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'y']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'd', 'z']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'e', 'a']
        );
        assert_eq!(
            pwd.next().unwrap(),
            ['c', 'q', 'j', 'x', 'j', 'n', 'e', 'b']
        );
    }

    #[test]
    fn solution() {
        let day = Day11::parse(INPUT);
        assert_eq!(day.fst(), ['c', 'q', 'j', 'x', 'x', 'y', 'z', 'z']);
        assert_eq!(day.snd(), ['c', 'q', 'k', 'a', 'a', 'b', 'c', 'c']);
    }
}
