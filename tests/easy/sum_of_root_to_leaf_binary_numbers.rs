// 1022. Sum of Root To Leaf Binary Numbers
// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let current = current * 2 + val;
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();
            if left.is_none() && right.is_none() {
                return current;
            }
            Self::dfs(left, current) + Self::dfs(right, current)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::sum_of_root_to_leaf_binary_numbers::Solution;
    use crate::shared::tree_node::create_tree;

    #[test]
    fn test_sum_root_to_leaf_1() {
        let root = create_tree(&[
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
        ]);
        assert_eq!(22, Solution::sum_root_to_leaf(root));
    }

    #[test]
    fn test_sum_root_to_leaf_2() {
        let root = create_tree(&[Some(0)]);
        assert_eq!(0, Solution::sum_root_to_leaf(root));
    }
}
