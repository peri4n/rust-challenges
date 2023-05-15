use std::collections::HashSet;

struct QuickFind {
    id: Vec<usize>,
}

impl QuickFind {
    pub fn new(capacity: usize) -> Self {
        let mut id = vec![0; capacity];
        for (i, val) in id.iter_mut().enumerate() {
            *val = i;
        }

        Self { id }
    }

    pub fn connect(&mut self, p: usize, q: usize) -> bool {
        let p_root = self.root(p);
        let q_root = self.root(q);

        if p_root != q_root {
            self.id[p_root] = q_root;
            true
        } else {
            false
        }
    }

    fn root(&self, q: usize) -> usize {
        let mut curr = q;

        while self.id[curr] != curr {
            curr = self.id[curr];
        }

        curr
    }

    pub fn groups(&self) -> usize {
        let mut set = HashSet::new();
        for &c in self.id.iter() {
            let root = self.root(c);
            set.insert(root);
        }
        set.len()
    }
}

fn similar(str1: &str, str2: &str) -> bool {
    let mut mismatches = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            mismatches += 1
        }

        if mismatches > 2 {
            return false;
        }
    }

    mismatches == 2 || mismatches == 0
}

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let mut find = QuickFind::new(strs.len());

    for i in 0..strs.len() {
        for j in i + 1..strs.len() {
            if similar(&strs[i], &strs[j]) {
                find.connect(i, j);
            }
        }
    }

    find.groups() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(
            num_similar_groups(vec![
                "tars".into(),
                "rats".into(),
                "arts".into(),
                "star".into()
            ]),
            2
        );
        assert_eq!(num_similar_groups(vec!["omv".into(), "ovm".into()]), 1);
        assert_eq!(num_similar_groups(vec!["abc".into(), "abc".into()]), 1);
    }
}
