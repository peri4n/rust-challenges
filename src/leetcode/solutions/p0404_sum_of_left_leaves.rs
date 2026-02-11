use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let mut sum = 0;

            if let Some(left) = &node.left {
                let left = left.borrow();
                if left.left.is_none() && left.right.is_none() {
                    sum += left.val;
                }
            }

            sum += sum_of_left_leaves(node.left.clone());
            sum += sum_of_left_leaves(node.right.clone());

            return sum;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode::new(
                3,
                Some(Rc::new(RefCell::new(TreeNode::new(9, None, None)))),
                Some(Rc::new(RefCell::new(TreeNode::new(
                    20,
                    Some(Rc::new(RefCell::new(TreeNode::new(15, None, None)))),
                    Some(Rc::new(RefCell::new(TreeNode::new(7, None, None))))
                ))))
            ))))),
            24
        );
    }
}
