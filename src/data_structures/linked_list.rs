use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    data: i32,
    next: Link,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Node { data, next: None }
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct LinkedList {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, data: i32) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        match self.tail.take() {
            None => self.head = Some(node.clone()),
            Some(t) => t.borrow_mut().next = Some(node.clone()),
        }
        self.tail = Some(node);
        self.length += 1;
    }

    pub fn prepend(&mut self, data: i32) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        match self.head.take() {
            Some(h) => node.borrow_mut().next = Some(h),
            None => self.tail = Some(node.clone()),
        }
        self.head = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.length = self.length.clamp(0, self.length - 1);
        match self.head.take() {
            Some(h) => {
                let d = h.borrow().data;
                self.head = h.borrow_mut().next.take();
                Some(d)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.prepend(4);

        assert!(list.pop().unwrap() == 4);
        assert!(list.pop().unwrap() == 1);
        assert!(list.length == 2);
        dbg!(list);
    }
}
