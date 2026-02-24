// 1382. Balance a Binary Search Tree
// https://leetcode.com/problems/balance-a-binary-search-tree/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Step 1: Perform in-order traversal to get sorted values
        let mut values = Vec::new();
        Self::inorder(&root, &mut values);

        // Step 2: Build balanced BST from sorted values
        Self::build_balanced_bst(&values, 0, values.len())
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder(&n.left, values);
            values.push(n.val);
            Self::inorder(&n.right, values);
        }
    }

    fn build_balanced_bst(
        values: &[i32],
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            return None;
        }

        let mid = (start + end - 1) / 2;
        let mut node = TreeNode::new(values[mid]);
        node.left = Self::build_balanced_bst(values, start, mid);
        node.right = Self::build_balanced_bst(values, mid + 1, end);

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::balance_a_binary_search_tree::Solution;
    use crate::shared::tree_node::create_tree;

    #[test]
    fn test_balance_bst_1() {
        let root = create_tree(&[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ]);
        let output = create_tree(&[Some(2), Some(1), Some(3), None, None, None, Some(4)]);
        assert_eq!(output, Solution::balance_bst(root));
    }

    #[test]
    fn test_balance_bst_2() {
        let root = create_tree(&[Some(2), Some(1), Some(3)]);
        let output = create_tree(&[Some(2), Some(1), Some(3)]);
        assert_eq!(output, Solution::balance_bst(root));
    }
}
