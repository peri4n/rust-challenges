use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct QuickFind {
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

    pub fn group_sizes(&self) -> Vec<usize> {
        let mut size_map: HashMap<usize, usize> = HashMap::new();
        for &c in self.id.iter() {
            let root = self.root(c);
            *size_map.entry(root).or_insert(0) += 1;
        }
        size_map
            .values()
            .into_iter()
            .cloned()
            .sorted()
            .rev()
            .collect()
    }
}
