// 2265. Count Nodes Equal to Average of Subtree
// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
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
