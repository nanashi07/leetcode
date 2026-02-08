// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::balanced_binary_tree::Solution;
    use crate::shared::create_tree;

    #[test]
    fn test_is_balanced_1() {
        let root = create_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(true, Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced_2() {
        let root = create_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert_eq!(false, Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced_3() {
        let root = create_tree(&[]);
        assert_eq!(true, Solution::is_balanced(root));
    }
}
