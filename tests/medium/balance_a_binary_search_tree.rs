// 1382. Balance a Binary Search Tree
// https://leetcode.com/problems/balance-a-binary-search-tree/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::balance_a_binary_search_tree::Solution;
    use crate::shared::create_tree;

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
