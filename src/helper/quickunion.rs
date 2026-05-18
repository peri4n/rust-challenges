use itertools::Itertools;

#[derive(Debug)]
pub struct QuickUnion {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl QuickUnion {
    pub fn new(capacity: usize) -> Self {
        let mut id = vec![0; capacity];
        for (i, val) in id.iter_mut().enumerate() {
            *val = i;
        }

        Self { id, size: vec![1; capacity], count: capacity }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn connect(&mut self, p: usize, q: usize) -> bool {
        let p_root = self.root_mut(p);
        let q_root = self.root_mut(q);

        if p_root != q_root {
            if self.size[p_root] < self.size[q_root] {
                self.id[p_root] = q_root;
                self.size[q_root] += self.size[p_root];
            } else {
                self.id[q_root] = p_root;
                self.size[p_root] += self.size[q_root];
            }
            self.count -= 1;
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

    /// This method not only computes the root but compresses the path to the root
    fn root_mut(&mut self, q: usize) -> usize {
        let mut curr = q;

        while self.id[curr] != curr {
            self.id[curr] = self.id[self.id[curr]];
            curr = self.id[curr];
        }

        curr
    }

    pub fn groups(&self) -> usize {
        self.count
    }

    pub fn group_sizes(&self) -> Vec<usize> {
        self.id.iter()
            .enumerate()
            .filter_map(|(i, &root)| (root == i).then_some(self.size[i]))
            .sorted()
            .rev()
            .collect()
    }
}
