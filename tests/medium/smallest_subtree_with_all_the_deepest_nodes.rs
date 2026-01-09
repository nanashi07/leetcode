// 865. Smallest Subtree with all the Deepest Nodes
// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root).0
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        match node {
            None => (None, 0),
            Some(node_rc) => {
                let node_ref = node_rc.borrow();
                let left = Self::dfs(node_ref.left.clone());
                let right = Self::dfs(node_ref.right.clone());

                // If left and right subtrees have the same depth,
                // current node is the LCA of all deepest nodes
                if left.1 == right.1 {
                    (Some(node_rc.clone()), left.1 + 1)
                } else if left.1 > right.1 {
                    // Deepest nodes are in the left subtree
                    (left.0, left.1 + 1)
                } else {
                    // Deepest nodes are in the right subtree
                    (right.0, right.1 + 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::shared::create_tree;

    #[test]
    fn test_subtree_with_all_deepest_1() {
        let root = create_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let output = create_tree(&[Some(2), Some(7), Some(4)]);
        assert_eq!(output, Solution::subtree_with_all_deepest(root));
    }

    #[test]
    fn test_subtree_with_all_deepest_2() {
        let root = create_tree(&[Some(1)]);
        let output = create_tree(&[Some(1)]);
        assert_eq!(output, Solution::subtree_with_all_deepest(root));
    }

    #[test]
    fn test_subtree_with_all_deepest_3() {
        let root = create_tree(&[Some(0), Some(1), Some(3), None, Some(2)]);
        let output = create_tree(&[Some(2)]);
        assert_eq!(output, Solution::subtree_with_all_deepest(root));
    }
}
