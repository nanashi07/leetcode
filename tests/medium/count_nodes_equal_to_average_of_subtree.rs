// 2265. Count Nodes Equal to Average of Subtree
// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).2
    }

    // Returns (sum, node_count, matches) for the subtree rooted at `node`.
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i64, i64, i32) {
        let Some(node_rc) = node else {
            return (0, 0, 0);
        };
        let node_ref = node_rc.borrow();
        let (ls, lc, lm) = Self::dfs(&node_ref.left);
        let (rs, rc, rm) = Self::dfs(&node_ref.right);

        let sum = ls + rs + node_ref.val as i64;
        let count = lc + rc + 1;
        let matches = lm + rm + (sum / count == node_ref.val as i64) as i32;
        (sum, count, matches)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_nodes_equal_to_average_of_subtree::Solution;
    use crate::shared::tree_node::create_tree;

    #[test]
    fn test_average_of_subtree_1() {
        let root = create_tree(&[Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)]);
        assert_eq!(5, Solution::average_of_subtree(root));
    }

    #[test]
    fn test_average_of_subtree_2() {
        let root = create_tree(&[Some(1)]);
        assert_eq!(1, Solution::average_of_subtree(root));
    }
}
