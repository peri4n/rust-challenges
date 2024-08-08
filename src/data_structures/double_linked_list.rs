use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Node {
    data: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct DoubleLinkedList {
    length: u64,
    head: Link,
    tail: Link,
}

impl DoubleLinkedList {
    pub fn new() -> Self {
        DoubleLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn prepend(&mut self, data: i32) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.take() {
            Some(h) => {
                self.head = Some(node.clone());
                h.borrow_mut().prev = Some(node.clone());
            },
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
        self.length += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::double_linked_list::DoubleLinkedList;

    #[test]
    fn test_prepend() {
        let mut list = DoubleLinkedList::new();
        list.prepend(1);
        assert!(list.head.as_ref().unwrap().borrow().data == 1);
        list.prepend(2);
        assert!(list.head.as_ref().unwrap().borrow().data == 2);
        list.prepend(3);
        assert!(list.head.as_ref().unwrap().borrow().data == 3);
        list.prepend(4);
        assert!(list.head.as_ref().unwrap().borrow().data == 4);
        list.prepend(5);
        assert!(list.head.as_ref().unwrap().borrow().data == 5);
        println!("{:?}", list.length);
    }
}
