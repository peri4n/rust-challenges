use crate::leetcode::utils::single_linked_list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_with_carry(l1, l2, false)
}

pub fn add_two_numbers_with_carry(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: bool,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => {
            if carry {
                Some(Box::new(ListNode::new(1)))
            } else {
                None
            }
        }
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val + if carry { 1 } else { 0 };
            let next = add_two_numbers_with_carry(n1.next, n2.next, sum >= 10);
            Some(Box::new(ListNode { val: sum % 10, next }))
        }
        (Some(n1), None) => {
            let sum = n1.val + if carry { 1 } else { 0 };
            let next = add_two_numbers_with_carry(n1.next, None, sum >= 10);
            Some(Box::new(ListNode { val: sum % 10, next }))
        }
        (None, Some(n2)) => {
            let sum = n2.val + if carry { 1 } else { 0 };
            let next = add_two_numbers_with_carry(None, n2.next, sum >= 10);
            Some(Box::new(ListNode { val: sum % 10, next }))
        }
    }
}

#[cfg(test)]
mod test {

    use crate::leetcode::utils::single_linked_list::ListNode;

    use super::add_two_numbers;

    fn from_vec(v: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &x in v.iter().rev() {
            head = Some(Box::new(ListNode { val: x, next: head }));
        }
        head
    }

    fn to_vec(mut l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(n) = l {
            out.push(n.val);
            l = n.next;
        }
        out
    }

    #[test]
    fn examples() {
        let num1 = from_vec(&[2, 3, 1]);
        let num2 = from_vec(&[9, 9]);
        let result = from_vec(&[1, 3, 2]);

        assert_eq!(add_two_numbers(None, num1.clone()), num1);
        assert_eq!(add_two_numbers(num1.clone(), None), num1);
        assert_eq!(add_two_numbers(num1.clone(), num2.clone()), result);
    }

    #[test]
    fn carry_over_long_lists() {
        let l1 = from_vec(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = from_vec(&[9, 9, 9, 9]);
        let res = add_two_numbers(l1, l2);
        assert_eq!(to_vec(res), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
