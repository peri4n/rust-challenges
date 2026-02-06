use crate::leetcode::utils::single_linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);

    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))
        );
    }
}
