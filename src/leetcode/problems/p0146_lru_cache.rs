use std::collections::HashMap;

const NOT_FOUND: i32 = -1;

/// A node in our index-based doubly-linked list.
/// Each node represents one cached key-value pair.
/// `prev` and `next` are indices into the `nodes` Vec, not pointers.
struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

/// LRU Cache using a HashMap + index-based doubly-linked list.
///
/// The linked list maintains access order: most-recently-used near the head,
/// least-recently-used near the tail. Two sentinel nodes (head at index 0,
/// tail at index 1) simplify insertion/removal by eliminating edge cases.
///
/// Layout: head <-> MRU <-> ... <-> LRU <-> tail
///
/// All operations (get, put) run in O(1) time.
pub struct LRUCache {

    /// Maps keys to their node index in `nodes`.
    map: HashMap<i32, usize>,

    /// Arena-style storage. Nodes are never removed from this Vec;
    /// they are only unlinked from the doubly-linked list on eviction.
    /// Index 0 = head sentinel, index 1 = tail sentinel.
    nodes: Vec<Node>,

    /// Index of the head sentinel (always 0).
    head: usize,

    /// Index of the tail sentinel (always 1).
    tail: usize,

    /// Maximum number of entries before eviction.
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        // Initialize with two sentinel nodes linked to each other:
        // head (idx 0) <-> tail (idx 1)
        let head_node = Node { key: 0, value: 0, prev: 0, next: 1 };
        let tail_node = Node { key: 0, value: 0, prev: 0, next: 1 };
        let mut cache = Self {
            map: HashMap::with_capacity(capacity as usize),
            nodes: vec![head_node, tail_node],
            head: 0,
            tail: 1,
            capacity: capacity as usize,
        };
        cache.nodes[0].next = 1;
        cache.nodes[1].prev = 0;
        cache
    }

    /// Unlinks a node from its current position in the list.
    /// After this call, the node's neighbors point to each other,
    /// bypassing the detached node.
    fn detach(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    /// Inserts a node right after the head sentinel, making it the
    /// most-recently-used entry.
    fn attach_after_head(&mut self, idx: usize) {
        let old_first = self.nodes[self.head].next;
        self.nodes[idx].prev = self.head;
        self.nodes[idx].next = old_first;
        self.nodes[self.head].next = idx;
        self.nodes[old_first].prev = idx;
    }

    /// Returns the value for `key`, or -1 if not present.
    /// Marks the entry as most-recently-used.
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.detach(idx);
            self.attach_after_head(idx);
            self.nodes[idx].value
        } else {
            NOT_FOUND
        }
    }

    /// Inserts or updates `key` with `value`.
    /// If the key already exists, updates the value and marks it as MRU.
    /// If the cache is at capacity, evicts the least-recently-used entry
    /// (the node just before the tail sentinel) before inserting.
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            // Key exists: update value in place, move to front
            self.nodes[idx].value = value;
            self.detach(idx);
            self.attach_after_head(idx);
        } else {
            // Key is new: evict LRU if at capacity
            if self.map.len() == self.capacity {
                let lru = self.nodes[self.tail].prev;
                self.map.remove(&self.nodes[lru].key);
                self.detach(lru);
                // Note: the evicted node remains in `nodes` but is unreachable
            }
            // Allocate new node at end of arena
            let idx = self.nodes.len();
            self.nodes.push(Node { key, value, prev: 0, next: 0 });
            self.map.insert(key, idx);
            self.attach_after_head(idx);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn capacity_one() {
        let mut c = LRUCache::new(1);
        c.put(1, 1);
        c.put(2, 2);
        assert_eq!(c.get(1), -1);
        assert_eq!(c.get(2), 2);
    }

    #[test]
    fn capacity_three() {
        let mut c = LRUCache::new(3);
        c.put(1, 1);
        c.put(2, 2);
        c.put(3, 3);
        c.put(4, 4);
        assert_eq!(c.get(1), -1);
        assert_eq!(c.get(2), 2);
        assert_eq!(c.get(3), 3);
        assert_eq!(c.get(4), 4);
    }
}
